use ::entity::user;
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_user(db: &DbConn, usr: user::Model) -> Result<user::Model, DbErr> {
        user::ActiveModel {
            name: Set(usr.name.to_owned()),
            email: Set(usr.email.to_owned()),
            password: Set(usr.password.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
        .unwrap()
        .try_into_model()
    }
}
