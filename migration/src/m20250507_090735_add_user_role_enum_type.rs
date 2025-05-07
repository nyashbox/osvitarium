use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    sea_orm::{DbBackend, Schema},
};

use sea_orm::{ActiveEnum, DeriveActiveEnum, EnumIter};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(DbBackend::Postgres);
        manager
            .create_type(schema.create_enum_from_active_enum::<UserRole>())
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().name(UserRole::name()).to_owned())
            .await
    }
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_role")]
pub enum UserRole {
    #[sea_orm(string_value = "Student")]
    Student,
    #[sea_orm(string_value = "Teacher")]
    Teacher,
    #[sea_orm(string_value = "Principal")]
    Principal,
}
