use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Principal::Table)
                    .if_not_exists()
                    .col(pk_auto(Principal::PrincipalId))
                    .col(integer(Principal::UserId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Principal::Table, Principal::UserId)
                            .to(User::Table, User::UserId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Principal::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Principal {
    Table,
    PrincipalId,
    UserId,
}

#[derive(DeriveIden)]
enum User {
    Table,
    UserId,
}
