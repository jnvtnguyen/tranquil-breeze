use ::entity::{
    user, user::Entity as User, workspace, workspace::Entity as Workspace, workspace_user,
    workspace_user::Entity as WorkspaceUser,
};
use sea_orm::*;
use serde::Serialize;

#[derive(Serialize, FromQueryResult)]
pub struct WorkspaceUserCombined {
    name: String,
    email: String,
    image: String,
    owner: bool,
    activated: bool,
}

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
        let result = Workspace::find()
            .find_also_related(workspace_user::Entity)
            .filter(
                Condition::all()
                    .add(workspace_user::Column::UserId.eq(user_id))
                    .add(workspace::Column::Slug.eq(slug)),
            )
            .one(db)
            .await?;

        Ok(result.map(|(workspace, _)| workspace))
    }

    pub async fn find_workspaces_by_user_id(
        db: &DbConn,
        user_id: i32,
    ) -> Result<Vec<workspace::Model>, DbErr> {
        let result = Workspace::find()
            .find_also_related(workspace_user::Entity)
            .filter(workspace_user::Column::UserId.eq(user_id))
            .all(db)
            .await?;

        let workspaces = result
            .into_iter()
            .map(|(workspace, _)| workspace)
            .collect::<Vec<_>>();

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

    pub async fn find_users_by_workspace_id(
        db: &DbConn,
        workspace_id: i32,
    ) -> Result<Vec<WorkspaceUserCombined>, DbErr> {
        let users = WorkspaceUser::find()
            .find_also_related(user::Entity)
            .filter(workspace_user::Column::WorkspaceId.eq(workspace_id))
            .all(db)
            .await?;

        let combined = users
            .into_iter()
            .map(|(workspace_user, user)| {
                let user = user.unwrap();

                WorkspaceUserCombined {
                    name: user.name,
                    email: user.email,
                    image: user.image,
                    owner: workspace_user.owner,
                    activated: workspace_user.activated,
                }
            })
            .collect::<Vec<_>>();

        Ok(combined)
    }
}
