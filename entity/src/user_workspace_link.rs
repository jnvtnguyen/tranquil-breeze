use super::{user, workspace, workspace_user};
use sea_orm::entity::prelude::*;

pub struct UserToWorkspaces;

impl Linked for UserToWorkspaces {
    type FromEntity = user::Entity;
    type ToEntity = workspace::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            workspace_user::Relation::User.def().rev(),
            workspace_user::Entity::belongs_to(workspace::Entity)
                .from(workspace_user::Column::WorkspaceId)
                .to(workspace::Column::Id)
                .into(),
        ]
    }
}
