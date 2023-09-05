use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WorkspaceUserActivation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WorkspaceUserActivation::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(WorkspaceUserActivation::Uuid)
                            .uuid()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WorkspaceUserActivation::WorkspaceUserId)
                            .integer()
                            .unique_key()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_workspace_user_activation_workspaceuserid")
                            .from(
                                entity::workspace_user_activation::Entity,
                                entity::workspace_user_activation::Column::WorkspaceUserId,
                            )
                            .to(
                                entity::workspace_user::Entity,
                                entity::workspace_user::Column::Id,
                            ),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(WorkspaceUserActivation::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum WorkspaceUserActivation {
    Table,
    Id,
    Uuid,
    WorkspaceUserId,
}
