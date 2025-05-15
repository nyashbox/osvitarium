pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_table;
mod m20250421_095451_create_principal_table;
mod m20250424_083640_create_student_table;
mod m20250424_085224_create_teacher_table;
mod m20250507_090735_add_user_role_enum_type;
mod m20250507_103403_user_add_role_field;
mod m20250509_160033_create_course_table;
mod m20250511_104740_make_role_not_nullable;
mod m20250511_162948_course_add_meeting_field;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user_table::Migration),
            Box::new(m20250421_095451_create_principal_table::Migration),
            Box::new(m20250424_083640_create_student_table::Migration),
            Box::new(m20250424_085224_create_teacher_table::Migration),
            Box::new(m20250507_090735_add_user_role_enum_type::Migration),
            Box::new(m20250507_103403_user_add_role_field::Migration),
            Box::new(m20250509_160033_create_course_table::Migration),
            Box::new(m20250511_104740_make_role_not_nullable::Migration),
            Box::new(m20250511_162948_course_add_meeting_field::Migration),
        ]
    }
}
