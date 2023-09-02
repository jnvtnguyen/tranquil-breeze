use axum::{extract::Extension, routing::get, Json, Router};
use serde::Serialize;

use entity::workspace;
use service::Query as QueryCore;

use crate::{extractor::AuthUser, ApiContext, Result};

pub fn router() -> Router {
    Router::new().route("/api/workspaces", get(workspaces))
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
