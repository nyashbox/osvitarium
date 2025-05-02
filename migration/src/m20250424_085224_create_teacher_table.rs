use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Teacher::Table)
                    .if_not_exists()
                    .col(pk_auto(Teacher::TeacherId))
                    .col(integer(Teacher::UserId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Teacher::Table, Teacher::UserId)
                            .to(User::Table, User::UserId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Teacher::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Teacher {
    Table,
    TeacherId,
    UserId,
}

#[derive(DeriveIden)]
enum User {
    Table,
    UserId,
}
