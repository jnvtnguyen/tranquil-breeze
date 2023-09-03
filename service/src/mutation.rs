use ::entity::{user, workspace, workspace_user};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_user(db: &DbConn, usr: user::Model) -> Result<user::ActiveModel, DbErr> {
        user::ActiveModel {
            name: Set(usr.name.to_owned()),
            email: Set(usr.email.to_owned()),
            password: Set(usr.password.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn create_workspace(
        db: &DbConn,
        wrkspace: workspace::Model,
        user_id: i32,
    ) -> Result<workspace::ActiveModel, DbErr> {
        let workspace = workspace::ActiveModel {
            name: Set(wrkspace.name.to_owned()),
            image: Set(wrkspace.image.to_owned()),
            slug: Set(wrkspace.slug.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await?;

        workspace_user::ActiveModel {
            user_id: Set(user_id),
            workspace_id: Set(workspace.id.to_owned().unwrap()),
            owner: Set(true),
            ..Default::default()
        }
        .save(db)
        .await?;

        Ok(workspace)
    }
}
