use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Course::Table)
                    .add_column(boolean(Course::IsRunningMeeting).default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Course::Table)
                    .drop_column(Course::IsRunningMeeting)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Course {
    Table,
    IsRunningMeeting,
}
