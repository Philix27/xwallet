use std::env::VarError;

pub struct AppEnv;

impl AppEnv {
    pub fn get_jwt_secret() -> String {
        std::env::var("JWT_SECRET").expect("JWT_secrete not found")
    }

    pub fn get_port() -> String {
        std::env::var("JWT_SECRET").unwrap_or("9000".to_string())
    }
    pub fn get_log_level() -> String {
        std::env::var("LOG_LEVEL").unwrap_or("INFO".to_string())
    }
    pub fn get_log_file() -> Result<std::string::String, VarError> {
        std::env::var("LOG_FILE")
    }
}
