use axum::{extract::Path, routing::post, Extension, Router};
use service::{Mutation as MutationCore, Query as QueryCore};
use uuid::Uuid;

use crate::{extractor::AuthUser, ApiContext, Error, Result};

pub fn router() -> Router {
    Router::new().route(
        "/api/activate-workspace/:workspace_user_activation_uuid",
        post(activate_workspace_user),
    )
}

async fn activate_workspace_user(
    ctx: Extension<ApiContext>,
    _auth_user: AuthUser,
    Path(workspace_user_activation_uuid): Path<Uuid>,
) -> Result<()> {
    let activation =
        QueryCore::find_workspace_user_activation_by_uuid(&ctx.db, workspace_user_activation_uuid)
            .await?;

    let activation = activation.ok_or(Error::NotFound)?;

    MutationCore::activate_workspace_user(&ctx.db, activation).await?;

    Ok(())
}
