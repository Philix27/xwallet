pub struct AppEnv {
    pub jwt_secret: String,
    pub hash_secret: String,
    pub postgres_db_url: String,
    pub port: String,
    pub log_file: String,
    pub log_level: String,
}

impl AppEnv {
    pub fn new() -> Self {
        AppEnv {
            jwt_secret: std::env::var("JWT_SECRET").expect("JWT_secrete not found"),
            hash_secret: std::env::var("HASH_SECRET").expect("Database URL required"),
            postgres_db_url: std::env::var("DATABASE_URL").expect("Database URL required"),
            port: std::env::var("PORT").unwrap_or("7000".to_string()),
            log_file: std::env::var("LOG_FILE").expect("Database URL required"),
            log_level: std::env::var("LOG_LEVEL").unwrap_or("INFO".to_string()),
        }
    }
}

const V: String = std::env::var("PORT").unwrap_or("7000".to_string());
