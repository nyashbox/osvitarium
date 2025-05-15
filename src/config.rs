use clap::Parser;

#[derive(Parser)]
pub struct AppConfig {
    #[arg(long, default_value = "0.0.0.0")]
    /// Host to listen on
    pub host: String,

    /// Port to listen on
    #[arg(long, default_value_t = 8080)]
    pub port: u16,

    /// Database connection URL
    #[arg(short, long, env = "OSVITARIUM_DATABASE_URL")]
    pub db: String,

    /// Jitsi application ID
    #[arg(long, env = "OSVITARIUM_JITSI_APP_ID")]
    pub jitsi_app_id: String,

    /// Jitsi secret
    #[arg(long, env = "OSVITARIUM_JITSI_SECRET")]
    pub jitsi_secret: String,

    #[arg(long, env = "OSVITARIUM_JITSI_KID")]
    pub jitsi_kid: String,

    /// Application secret
    #[arg(short, long, env = "OSVITARIUM_SECRET")]
    pub secret: String,
}
