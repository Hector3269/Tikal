pub fn get_env_var(key: &str) -> Option<String> {
    std::env::var(key).ok()
}