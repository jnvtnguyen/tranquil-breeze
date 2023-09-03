use axum::{
    extract::{Extension, Path},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use entity::workspace;
use service::{sea_orm::SqlErr, Mutation as MutationCore, Query as QueryCore};

use crate::{extractor::AuthUser, ApiContext, Error, Result};

pub fn router() -> Router {
    Router::new()
        .route("/api/workspaces/create", post(create_workspace))
        .route("/api/workspaces", get(workspaces))
        .route("/api/workspaces/:workspace_slug", get(workspace))
        .route("/api/workspaces/check-slug", post(check_slug))
}

#[derive(Serialize, Deserialize)]
struct WorkspaceBody<T> {
    workspace: T,
}

#[derive(Serialize, Deserialize)]
struct Workspace {
    name: String,
    image: String,
    slug: String,
}

async fn create_workspace(
    ctx: Extension<ApiContext>,
    auth_user: AuthUser,
    Json(req): Json<WorkspaceBody<workspace::Model>>,
) -> Result<Json<WorkspaceBody<Workspace>>> {
    let workspace = MutationCore::create_workspace(&ctx.db, req.workspace, auth_user.user_id).await;

    let response = match workspace {
        Ok(workspace) => {
            let workspace = Workspace {
                name: workspace.name.unwrap(),
                image: workspace.image.unwrap(),
                slug: workspace.slug.unwrap(),
            };
            Ok(Json(WorkspaceBody { workspace }))
        }
        Err(e) => {
            if let Some(err) = e.sql_err() {
                match err {
                    SqlErr::UniqueConstraintViolation(_) => {
                        Err(Error::unprocessable_entity([("", "")]))
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

async fn workspace(
    ctx: Extension<ApiContext>,
    auth_user: AuthUser,
    Path(workspace_slug): Path<String>,
) -> Result<Json<WorkspaceBody<workspace::Model>>> {
    let workspace =
        QueryCore::find_workspace_by_user_slug(&ctx.db, &workspace_slug, auth_user.user_id).await;

    let response = match workspace {
        Ok(workspace) => {
            if workspace.is_some() {
                Ok(Json(WorkspaceBody {
                    workspace: workspace.unwrap(),
                }))
            } else {
                Err(Error::NotFound)
            }
        }
        Err(e) => Err(Error::SeaOrm(e)),
    };

    response
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
