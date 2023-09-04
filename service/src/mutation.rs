use ::entity::{user, workspace, workspace_user};
use sea_orm::*;

pub struct Mutation;
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub image: String,
}
pub struct CreateWorkspace {
    pub name: String,
    pub slug: String,
    pub image: String,
}

impl Mutation {
    pub async fn create_user(db: &DbConn, usr: CreateUser) -> Result<user::ActiveModel, DbErr> {
        user::ActiveModel {
            name: Set(usr.name.to_owned()),
            email: Set(usr.email.to_owned()),
            password: Set(usr.password.to_owned()),
            image: Set(usr.image.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn create_workspace(
        db: &DbConn,
        wrkspace: CreateWorkspace,
        user_id: i32,
    ) -> Result<workspace::ActiveModel, DbErr> {
        let transaction = db.begin().await?;

        let workspace = workspace::ActiveModel {
            name: Set(wrkspace.name.to_owned()),
            slug: Set(wrkspace.slug.to_owned()),
            image: Set(wrkspace.image.to_owned()),
            ..Default::default()
        }
        .save(&transaction)
        .await?;

        workspace_user::ActiveModel {
            user_id: Set(user_id),
            workspace_id: Set(workspace.id.to_owned().unwrap()),
            owner: Set(true),
            activated: Set(true),
            ..Default::default()
        }
        .save(&transaction)
        .await?;

        transaction.commit().await?;

        Ok(workspace)
    }
}
