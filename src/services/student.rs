pub trait StudentService {}

impl StudentService for sea_orm::DatabaseConnection {}

#[cfg(test)]
mod tests {}
