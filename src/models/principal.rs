use entity::{principal::Model as PrincipalModel, user::Model as UserModel};

/// Structure representing principal model
#[derive(Debug, Clone)]
pub struct Principal {
    /// Database model of the user data
    pub user_model: UserModel,

    /// Database model of the principal
    pub principal_model: PrincipalModel,
}

impl Principal {}
