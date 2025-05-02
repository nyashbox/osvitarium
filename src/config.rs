use clap::Parser;

#[derive(Parser)]
pub struct AppConfig {
    #[arg(long, default_value = "0.0.0.0")]
    /// Host to listen on
    pub host: String,

    /// Port to listen on
    #[arg(long, default_value_t = 8080)]
    pub port: u16,

    /// Path to the configuration storage
    #[arg(long, default_value = "/etc/osvitarium.conf")]
    pub config: String,
}
