use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WorkspaceUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WorkspaceUser::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(WorkspaceUser::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_workspace_user_userid")
                            .from(
                                entity::workspace_user::Entity,
                                entity::workspace_user::Column::UserId,
                            )
                            .to(entity::user::Entity, entity::user::Column::Id),
                    )
                    .col(
                        ColumnDef::new(WorkspaceUser::WorkspaceId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_workspace_user_workspaceid")
                            .from(
                                entity::workspace_user::Entity,
                                entity::workspace_user::Column::WorkspaceId,
                            )
                            .to(entity::workspace::Entity, entity::workspace::Column::Id),
                    )
                    .col(ColumnDef::new(WorkspaceUser::Owner).boolean().not_null())
                    .col(
                        ColumnDef::new(WorkspaceUser::Activated)
                            .boolean()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_workspace_user_userid_workspaceid")
                    .table(WorkspaceUser::Table)
                    .col(WorkspaceUser::UserId)
                    .col(WorkspaceUser::WorkspaceId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WorkspaceUser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum WorkspaceUser {
    Table,
    Id,
    UserId,
    WorkspaceId,
    Owner,
    Activated,
}
