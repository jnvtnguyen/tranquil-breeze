use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    #[sea_orm(column_name = "name")]
    pub name: String,
    #[sea_orm(column_name = "email")]
    #[sea_orm(unique)]
    pub email: String,
    #[sea_orm(column_name = "password")]
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::workspace_user::Entity")]
    WorkspaceUser,
}

impl Related<super::workspace_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WorkspaceUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
