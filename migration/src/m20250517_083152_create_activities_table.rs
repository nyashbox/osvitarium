use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
    sea_orm::{DbBackend, Schema},
};

use sea_orm::{ActiveEnum, DeriveActiveEnum, EnumIter};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Step 1: Create enum to describe activity type
        let schema = Schema::new(DbBackend::Postgres);
        manager
            .create_type(schema.create_enum_from_active_enum::<ActivityType>())
            .await?;

        // Step 2: Create 'Activity' table
        manager
            .create_table(
                Table::create()
                    .table(Activity::Table)
                    .if_not_exists()
                    .col(pk_auto(Activity::AcitvityId))
                    .col(string(Activity::Title))
                    .col(string_null(Activity::Description))
                    .col(custom(Activity::Type, ActivityType::name()))
                    .col(integer(Activity::CourseId).not_null())
                    .col(timestamp(Activity::PublishedAt).default(Expr::current_timestamp()))
                    .col(timestamp_null(Activity::Deadline))
                    .col(integer_null(Activity::Points))
                    .col(boolean(Activity::IsHidden).default(false))
                    .col(integer(Activity::AuthorId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Activity::Table, Activity::CourseId)
                            .to(Course::Table, Course::CourseId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Activity::Table, Activity::AuthorId)
                            .to(User::Table, User::UserId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Step 1: Drop 'Activity' table
        manager
            .drop_table(Table::drop().table(Activity::Table).to_owned())
            .await?;

        // Step 2: Drop created custom type
        manager
            .drop_type(Type::drop().name(ActivityType::name()).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Activity {
    Table,
    AcitvityId,
    Title,
    Description,
    Type,
    CourseId,
    PublishedAt,
    Deadline,
    Points,
    IsHidden,
    AuthorId,
}

#[derive(DeriveIden)]
enum Course {
    Table,
    CourseId,
}

#[derive(DeriveIden)]
enum User {
    Table,
    UserId,
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "activity_type")]
pub enum ActivityType {
    #[sea_orm(string_value = "Material")]
    Material,
    #[sea_orm(string_value = "Assignment")]
    Assignment,
    #[sea_orm(string_value = "Test")]
    Test,
    #[sea_orm(string_value = "Upload")]
    Upload,
}
