use anyhow::Context;
use argon2::{password_hash::SaltString, Argon2, PasswordHash};
use axum::{extract::Extension, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use entity::user;
use service::{Mutation as MutationCore, Query as QueryCore};

use crate::{ApiContext, Result};

pub fn router() -> Router {
    Router::new()
        .route("/api/users/signup", post(create_user))
        .route("/api/users/check-email", post(check_email))
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
    req: Json<CheckEmailBody>,
) -> Result<Json<CheckEmail>> {
    let user = QueryCore::find_user_by_email(&ctx.db, &req.email).await;

    Ok(Json(CheckEmail { valid: false }))
}

#[derive(Serialize, Deserialize)]
struct UserBody<T> {
    user: T,
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}

async fn create_user(
    ctx: Extension<ApiContext>,
    Json(mut req): Json<UserBody<user::Model>>,
) -> Result<Json<UserBody<User>>> {
    let password_hash = hash_password(req.user.password).await?;
    req.user.password = password_hash;

    let user = MutationCore::create_user(&ctx.db, req.user.clone())
        .await
        .unwrap();

    Ok(Json(UserBody {
        user: User {
            name: user.name,
            email: user.email,
        },
    }))
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
