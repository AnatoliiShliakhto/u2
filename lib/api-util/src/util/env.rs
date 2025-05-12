pub fn init() {
    dotenv::dotenv().ok();
}

pub fn get_var(key: &str) -> Option<String> {
    dotenv::var(key).ok()
}

pub fn get_var_or_default(key: &str, default: &str) -> String {
    dotenv::var(key).unwrap_or(default.to_string())
}