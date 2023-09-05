use anyhow::Context;
use argon2::{password_hash::SaltString, Argon2, PasswordHash};
use axum::{
    extract::Extension,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use service::{sea_orm::SqlErr, CreateUser, Mutation as MutationCore, Query as QueryCore};

use crate::extractor::AuthUser;
use crate::{ApiContext, Error, Result};

pub fn router() -> Router {
    Router::new()
        .route("/api/users/signup", post(create_user))
        .route("/api/users/login", post(login_user))
        .route("/api/users/check-email", post(check_email))
        .route("/api/user", get(get_user))
}

#[derive(Deserialize)]
struct CheckEmailBody {
    email: String,
}

#[derive(Serialize)]
struct CheckEmail {
    valid: bool,
}

async fn check_email(
    ctx: Extension<ApiContext>,
    Json(req): Json<CheckEmailBody>,
) -> Result<Json<CheckEmail>> {
    let user = QueryCore::find_user_by_email(&ctx.db, &req.email).await;

    let response = match user {
        Ok(user) => {
            let valid: bool = user.is_none();
            Ok(Json(CheckEmail { valid }))
        }
        Err(e) => Err(Error::SeaOrm(e)),
    };

    response
}

#[derive(Serialize, Deserialize)]
struct UserBody<T> {
    user: T,
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
    token: String,
}

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
    password: String,
}

async fn create_user(
    ctx: Extension<ApiContext>,
    Json(req): Json<UserBody<CreateUserRequest>>,
) -> Result<Json<UserBody<User>>> {
    let mut user: CreateUser = CreateUser {
        name: req.user.name.to_owned(),
        email: req.user.email.to_owned(),
        password: req.user.password.to_owned(),
        image: "".to_owned(),
    };

    let password_hash = hash_password(req.user.password).await?;
    user.password = password_hash;

    let user = MutationCore::create_user(&ctx.db, user).await;
    let response: Result<Json<UserBody<User>>> = match user {
        Ok(user) => {
            let user = User {
                name: user.name.unwrap(),
                email: user.email.unwrap(),
                token: AuthUser {
                    user_id: user.id.unwrap(),
                }
                .to_jwt(&ctx),
            };
            Ok(Json(UserBody { user }))
        }
        Err(e) => {
            if let Some(err) = e.sql_err() {
                match err {
                    SqlErr::UniqueConstraintViolation(_) => {
                        Err(Error::unprocessable_entity([("email", "already exists")]))
                    }
                    _ => Err(Error::SeaOrm(e)),
                }
            } else {
                Err(Error::SeaOrm(e))
            }
        }
    };

    response
}

#[derive(Deserialize)]
struct LoginUser {
    email: String,
    password: String,
}

async fn login_user(
    ctx: Extension<ApiContext>,
    Json(req): Json<UserBody<LoginUser>>,
) -> Result<Json<UserBody<User>>> {
    let user = QueryCore::find_user_by_email(&ctx.db, &req.user.email).await?;

    if user.is_some() {
        let user = user.unwrap();
        verify_password(req.user.password, user.password).await?;

        Ok(Json(UserBody {
            user: User {
                name: user.name,
                email: user.email,
                token: AuthUser { user_id: user.id }.to_jwt(&ctx),
            },
        }))
    } else {
        Err(Error::Unauthorized)
    }
}

async fn get_user(auth_user: AuthUser, ctx: Extension<ApiContext>) -> Result<Json<UserBody<User>>> {
    let user = QueryCore::find_user_by_id(&ctx.db, auth_user.user_id).await?;

    if user.is_some() {
        let user = user.unwrap();
        Ok(Json(UserBody {
            user: User {
                name: user.name,
                email: user.email,
                token: auth_user.to_jwt(&ctx),
            },
        }))
    } else {
        Err(Error::Unauthorized)
    }
}

async fn hash_password(password: String) -> Result<String> {
    Ok(tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(PasswordHash::generate(Argon2::default(), password, &salt)
            .map_err(|e| anyhow::anyhow!("Failed to generate password hash: {}", e))?
            .to_string())
    })
    .await
    .context("Panic in generating hash")??)
}

async fn verify_password(password: String, password_hash: String) -> Result<()> {
    Ok(tokio::task::spawn_blocking(move || -> Result<()> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("Failed to parse password hash: {}", e))?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => Error::Unauthorized,
                _ => anyhow::anyhow!("Failed to verify password hash: {}", e).into(),
            })
    })
    .await
    .context("Panic in verifying password")??)
}
