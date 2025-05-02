pub trait PrincipalService {}

impl PrincipalService for sea_orm::DatabaseConnection {}

#[cfg(test)]
mod tests {}
