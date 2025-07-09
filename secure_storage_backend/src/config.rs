#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_max_age: i64,
    pub port: u16
}

impl Config {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_max_age = std::env::var("JWT_MAX_AGE").expect("JWT_MAX_AGE must be set");
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse::<u16>()
            .expect("PORT must be a valid number");
        Self {
            database_url,
            jwt_secret,
            jwt_max_age: jwt_max_age.parse::<i64>().expect("JWT_MAX_AGE must be a valid number"),
            port,
        }
    }
}