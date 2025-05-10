use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 'Course' table
        manager
            .create_table(
                Table::create()
                    .table(Course::Table)
                    .if_not_exists()
                    .to_owned()
                    .col(pk_auto(Course::CourseId))
                    .col(string_uniq(Course::Title))
                    .col(string_null(Course::Description))
                    .col(boolean(Course::IsActive).default(true))
                    .col(timestamp(Course::CreatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        // Course <-> Student linking table
        manager
            .create_table(
                Table::create()
                    .table(CourseStudent::Table)
                    .if_not_exists()
                    .col(integer(CourseStudent::CourseId))
                    .col(integer(CourseStudent::StudentId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(CourseStudent::Table, CourseStudent::CourseId)
                            .to(Course::Table, Course::CourseId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CourseStudent::Table, CourseStudent::StudentId)
                            .to(Student::Table, Student::StudentId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(CourseStudent::CourseId)
                            .col(CourseStudent::StudentId),
                    )
                    .to_owned(),
            )
            .await?;

        // Course <-> Instructor linking table
        manager
            .create_table(
                Table::create()
                    .table(CourseInstructor::Table)
                    .if_not_exists()
                    .col(integer(CourseInstructor::CourseId).not_null())
                    .col(integer(CourseInstructor::InstructorId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(CourseInstructor::Table, CourseInstructor::CourseId)
                            .to(Course::Table, Course::CourseId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CourseInstructor::Table, CourseInstructor::InstructorId)
                            .to(Teacher::Table, Teacher::TeacherId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(CourseInstructor::CourseId)
                            .col(CourseInstructor::InstructorId),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CourseStudent::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(CourseInstructor::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Course::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Course {
    Table,
    CourseId,
    Title,
    Description,
    IsActive,
    CreatedAt,
}

#[derive(DeriveIden)]
enum CourseInstructor {
    Table,
    CourseId,
    InstructorId,
}

#[derive(DeriveIden)]
enum CourseStudent {
    Table,
    CourseId,
    StudentId,
}

#[derive(DeriveIden)]
enum Teacher {
    Table,
    TeacherId,
}

#[derive(DeriveIden)]
enum Student {
    Table,
    StudentId,
}
