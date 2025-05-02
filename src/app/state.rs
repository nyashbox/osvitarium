/// Represents application state
pub struct AppState<S> {
    /// Database connection
    pub db: S,

    /// Application secret (for JWT encoding)
    pub secret: String,
}
