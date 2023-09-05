use sea_orm::entity::prelude::*;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize)]
#[sea_orm(table_name = "workspace_user_activation")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    #[sea_orm(unique)]
    pub uuid: Uuid,
    #[sea_orm(unique)]
    pub workspace_user_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    WorkspaceUser,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::WorkspaceUser => Entity::belongs_to(super::workspace_user::Entity)
                .from(Column::WorkspaceUserId)
                .to(super::workspace_user::Column::Id)
                .into(),
        }
    }
}

impl Related<super::workspace_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WorkspaceUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
