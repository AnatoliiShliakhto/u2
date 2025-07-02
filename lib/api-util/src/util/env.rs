use ::std::env;

pub fn get_var(key: &str) -> Option<String> {
    env::var(key).ok()
}

pub fn get_var_or_default(key: &str, default: &'static str) -> &'static str {
    if let Ok(value) = env::var(key) {
        Box::leak(value.into_boxed_str())    
    } else {
        default    
    }
}
