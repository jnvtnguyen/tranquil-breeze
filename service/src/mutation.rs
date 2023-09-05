use ::entity::{user, workspace, workspace_user, workspace_user_activation};
use sea_orm::*;
use uuid::Uuid;

use crate::WorkspaceUserCombined;

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

    pub async fn create_workspace_user(
        db: &DbConn,
        workspace_id: i32,
        user_id: i32,
    ) -> Result<WorkspaceUserCombined, DbErr> {
        let transaction = db.begin().await?;

        let workspace_user = workspace_user::ActiveModel {
            user_id: Set(user_id),
            workspace_id: Set(workspace_id),
            owner: Set(false),
            activated: Set(false),
            ..Default::default()
        }
        .save(&transaction)
        .await?;

        workspace_user_activation::ActiveModel {
            uuid: Set(Uuid::new_v4()),
            workspace_user_id: Set(workspace_user.id.to_owned().unwrap()),
            ..Default::default()
        }
        .save(&transaction)
        .await?;

        transaction.commit().await?;

        let user = user::Entity::find_by_id(user_id).one(db).await?.unwrap();

        let combined = WorkspaceUserCombined {
            name: user.name,
            email: user.email,
            image: user.image,
            owner: workspace_user.owner.unwrap(),
            activated: workspace_user.activated.unwrap(),
        };

        Ok(combined)
    }

    pub async fn activate_workspace_user(
        db: &DbConn,
        activation: workspace_user_activation::Model,
    ) -> Result<(), DbErr> {
        let transaction = db.begin().await?;

        workspace_user::ActiveModel {
            id: Set(activation.workspace_user_id),
            activated: Set(true),
            ..Default::default()
        }
        .save(&transaction)
        .await?;

        activation.into_active_model().delete(&transaction).await?;

        transaction.commit().await?;

        Ok(())
    }
}
