/// Represents application state
pub struct AppState<S> {
    /// Database connection
    pub db: S,

    /// Application secret (for JWT encoding)
    pub secret: String,

    /// Jitsi application ID
    pub jitsi_app_id: String,

    /// Jitsi secret
    pub jitsi_secret: String,

    /// Jitsi key ID
    pub jitsi_kid: String,
}
