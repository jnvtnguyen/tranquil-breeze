use axum::{
    extract::{Extension, Path},
    http::Request,
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use entity::workspace;
use service::{
    sea_orm::SqlErr, CreateWorkspace, Mutation as MutationCore, Query as QueryCore,
    WorkspaceUserCombined,
};

use crate::{extractor::AuthUser, ApiContext, Error, Result};

fn workspace_router() -> Router {
    Router::new()
        .route("/api/workspaces/:workspace_slug", get(workspace))
        .route("/api/workspaces/:workspace_slug/users", get(users))
        .route(
            "/api/workspaces/:workspace_slug/users/create",
            post(create_workspace_user),
        )
        .route_layer(middleware::from_fn(workspace_middleware))
}

pub fn router() -> Router {
    Router::new()
        .route("/api/workspaces", get(workspaces))
        .route("/api/workspaces/create", post(create_workspace))
        .route("/api/workspaces/check-slug", post(check_slug))
        .merge(workspace_router())
}

#[derive(Serialize, Deserialize)]
struct WorkspaceBody<T> {
    workspace: T,
}

#[derive(Serialize, Deserialize)]
struct Workspace {
    name: String,
    slug: String,
}

#[derive(Deserialize)]
struct CreateWorkspaceRequest {
    name: String,
    slug: String,
}

async fn create_workspace(
    ctx: Extension<ApiContext>,
    auth_user: AuthUser,
    Json(req): Json<WorkspaceBody<CreateWorkspaceRequest>>,
) -> Result<Json<WorkspaceBody<Workspace>>> {
    let workspace: CreateWorkspace = CreateWorkspace {
        name: req.workspace.name.to_owned(),
        slug: req.workspace.slug.to_owned(),
        image: "".to_owned(),
    };

    let workspace = MutationCore::create_workspace(&ctx.db, workspace, auth_user.user_id).await;

    let response = match workspace {
        Ok(workspace) => {
            let workspace = Workspace {
                name: workspace.name.unwrap(),
                slug: workspace.slug.unwrap(),
            };
            Ok(Json(WorkspaceBody { workspace }))
        }
        Err(e) => {
            if let Some(err) = e.sql_err() {
                match err {
                    SqlErr::UniqueConstraintViolation(_) => {
                        Err(Error::unprocessable_entity([("slug", "already exists")]))
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

async fn workspace_middleware<B>(
    ctx: Extension<ApiContext>,
    auth_user: AuthUser,
    Path(workspace_slug): Path<String>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let workspace =
        QueryCore::find_workspace_by_user_slug(&ctx.db, &workspace_slug, auth_user.user_id).await;

    let response = match workspace {
        Ok(workspace) => {
            if workspace.is_some() {
                req.extensions_mut().insert(workspace.unwrap());
                Ok(next.run(req).await)
            } else {
                Err(Error::NotFound)
            }
        }
        Err(e) => Err(Error::SeaOrm(e)),
    };

    response
}

async fn workspace(
    Extension(workspace): Extension<workspace::Model>,
) -> Result<Json<WorkspaceBody<workspace::Model>>> {
    Ok(Json(WorkspaceBody { workspace }))
}

#[derive(Deserialize)]
struct CheckSlugBody {
    slug: String,
}

#[derive(Serialize)]
struct CheckSlug {
    valid: bool,
}

async fn check_slug(
    ctx: Extension<ApiContext>,
    _auth_user: AuthUser,
    Json(req): Json<CheckSlugBody>,
) -> Result<Json<CheckSlug>> {
    let workspace = QueryCore::find_workspace_by_slug(&ctx.db, &req.slug).await;

    let response = match workspace {
        Ok(workspace) => {
            let valid: bool = workspace.is_none();
            Ok(Json(CheckSlug { valid }))
        }
        Err(e) => Err(Error::SeaOrm(e)),
    };

    response
}

#[derive(Serialize)]
struct WorkspacesBody<T> {
    workspaces: Vec<T>,
}

async fn workspaces(
    ctx: Extension<ApiContext>,
    auth_user: AuthUser,
) -> Result<Json<WorkspacesBody<workspace::Model>>> {
    let workspaces = QueryCore::find_workspaces_by_user_id(&ctx.db, auth_user.user_id).await?;

    Ok(Json(WorkspacesBody { workspaces }))
}

#[derive(Serialize)]
struct UsersBody<T> {
    users: Vec<T>,
}

async fn users(
    ctx: Extension<ApiContext>,
    Extension(workspace): Extension<workspace::Model>,
) -> Result<Json<UsersBody<WorkspaceUserCombined>>> {
    let users = QueryCore::find_users_by_workspace_id(&ctx.db, workspace.id).await?;

    Ok(Json(UsersBody { users }))
}

#[derive(Deserialize)]
struct CreateWorkspaceUserRequest {
    email: String,
}

#[derive(Serialize)]
struct UserBody<T> {
    user: T,
}

async fn create_workspace_user(
    ctx: Extension<ApiContext>,
    Extension(workspace): Extension<workspace::Model>,
    Json(req): Json<CreateWorkspaceUserRequest>,
) -> Result<Json<UserBody<WorkspaceUserCombined>>> {
    let user = QueryCore::find_user_by_email(&ctx.db, &req.email).await?;

    if user.is_none() {
        return Err(Error::NotFound);
    }

    let user = user.unwrap();
    let workspace_user = MutationCore::create_workspace_user(&ctx.db, workspace.id, user.id).await;

    let response = match workspace_user {
        Ok(workspace_user) => Ok(Json(UserBody {
            user: workspace_user,
        })),
        Err(e) => {
            if let Some(err) = e.sql_err() {
                match err {
                    SqlErr::UniqueConstraintViolation(_) => {
                        Err(Error::unprocessable_entity([("user", "already exists")]))
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
