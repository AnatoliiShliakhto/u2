use ::std::env;

pub fn get_var(key: &str) -> Option<String> {
    env::var(key).ok()
}

pub fn get_var_or_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or(default.to_string())
}
