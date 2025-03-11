use std::env;

pub fn get_env_var(name: &str) -> Result<String, String> {
    env::var(name).map_err(|_| format!("Missing required {}", name))
}