use ::entity::{user, user::Entity as User, user_workspace_link, workspace};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_user_by_email(
        db: &DbConn,
        email: &str,
    ) -> Result<Option<user::Model>, DbErr> {
        User::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await
    }

    pub async fn find_user_by_id(db: &DbConn, id: i32) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }

    pub async fn find_workspaces_by_user_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Vec<workspace::Model>, DbErr> {
        let result = User::find_by_id(id)
            .find_also_linked(user_workspace_link::UserToWorkspaces)
            .one(db)
            .await?;

        let workspaces: Vec<workspace::Model> = result
            .into_iter()
            .filter_map(|(_, workspace)| workspace)
            .collect();

        Ok(workspaces)
    }
}
