/// App configuration
///
/// Passed via command line, or environment variables.
use clap::{builder::NonEmptyStringValueParser, Parser};
use lib_types::shared::core::ExecEnv;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[clap(long, env = "EXEC_ENV")]
    pub exec_env: ExecEnv,

    /// The postgres connection URL without database name
    #[clap(long, env = "DATABASE_URL", value_parser = NonEmptyStringValueParser::new())]
    pub database_url: String,

    /// The app database name
    #[clap(long, env = "DB_NAME", value_parser = NonEmptyStringValueParser::new())]
    pub db_name: String,

    /// The API host
    #[clap(long, env = "APP_API_HOST", value_parser = NonEmptyStringValueParser::new())]
    pub api_host: String,

    /// The API port
    #[clap(long, env = "APP_API_PORT", value_parser = clap::value_parser!(u16).range(1..))]
    pub api_port: u16,

    /// Comma separated list of authorized referrers allowed to access the API
    #[clap(long, env = "APP_API_CORS", value_parser = NonEmptyStringValueParser::new(), value_delimiter = ',')]
    pub api_cors: Vec<String>,

    #[clap(long, env = "APP_WEB_URL", value_parser = NonEmptyStringValueParser::new())]
    pub app_web_url: String,

    /// Secret used for generating auth tokens
    #[clap(long, env = "APP_AUTH_SECRET", value_parser = NonEmptyStringValueParser::new())]
    pub app_auth_secret: String,

    /// Shared secret for confirmation tokens
    #[clap(long, env = "CONFIRM_SHARED_SECRET", value_parser = NonEmptyStringValueParser::new())]
    pub confirm_shared_secret: String,

    /// API key for sending email via SendGrid
    #[clap(long, env = "SENDGRID_API_KEY", value_parser = NonEmptyStringValueParser::new())]
    pub sendgrid_api_key: String,

    /// S3 endpoint
    #[clap(long, env = "S3_URL", value_parser = NonEmptyStringValueParser::new())]
    pub s3_url: String,

    /// S3 access key ID
    #[clap(long, env = "S3_ACCESS_KEY_ID", value_parser = NonEmptyStringValueParser::new())]
    pub s3_access_key_id: String,

    /// S3 secret key
    #[clap(long, env = "S3_SECRET_ACCESS_KEY", value_parser = NonEmptyStringValueParser::new())]
    pub s3_secret_access_key: String,
}
