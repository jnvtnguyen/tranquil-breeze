use ::entity::{user, user::Entity as User};
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
}
