/// App configuration
///
/// Passed via command line, or environment variables.
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// The postgres connection URL
    #[clap(long, env = "DATABASE_URL")]
    pub database_url: String,

    /// The app database name
    #[clap(long, env = "DB_NAME")]
    pub db_name: String,

    /// The API host
    #[clap(long, env = "API_TEST_HELPER_HOST")]
    pub api_host: String,

    /// The API port
    #[clap(long, env = "API_TEST_HELPER_PORT")]
    pub api_port: u16,
}
