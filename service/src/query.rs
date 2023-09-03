use ::entity::{
    user, user::Entity as User, user_workspace_link, workspace, workspace::Entity as Workspace,
};
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

    pub async fn find_workspace_by_user_slug(
        db: &DbConn,
        slug: &str,
        user_id: i32,
    ) -> Result<Option<workspace::Model>, DbErr> {
        let result = User::find_by_id(user_id)
            .find_also_linked(user_workspace_link::UserToWorkspaces)
            .all(db)
            .await?;

        let workspace: Option<workspace::Model> = result
            .into_iter()
            .filter_map(|(_, workspace)| workspace)
            .find(|workspace| workspace.slug == slug);

        Ok(workspace)
    }

    pub async fn find_workspaces_by_user_id(
        db: &DbConn,
        user_id: i32,
    ) -> Result<Vec<workspace::Model>, DbErr> {
        let result = User::find_by_id(user_id)
            .find_also_linked(user_workspace_link::UserToWorkspaces)
            .all(db)
            .await?;

        let workspaces: Vec<workspace::Model> = result
            .into_iter()
            .filter_map(|(_, workspace)| workspace)
            .collect();

        Ok(workspaces)
    }

    pub async fn find_workspace_by_slug(
        db: &DbConn,
        slug: &str,
    ) -> Result<Option<workspace::Model>, DbErr> {
        Workspace::find()
            .filter(workspace::Column::Slug.eq(slug))
            .one(db)
            .await
    }
}
