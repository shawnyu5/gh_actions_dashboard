use cached::lazy_static::lazy_static;

pub struct Environment {
    pub api_address: String,
}

lazy_static! {
    pub static ref ENVIRONMENT: Environment = get_environment();
}

/// load all enviroment variables from .env
/// return: Environment struct
fn get_environment() -> Environment {
    // dotenv().unwrap();
    return Environment {
        api_address: std::env!("API_ADDRESS").to_string(),
    };
    // .unwrap_or("https://gh-actions-dashboard-api.fly.dev".to_string()),
}
