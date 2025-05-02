pub trait TeacherService {}

impl TeacherService for sea_orm::DatabaseConnection {}

#[cfg(test)]
mod tests {}
