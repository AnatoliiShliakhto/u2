pub struct AppConfig {
    pub name: &'static str,
    pub version: &'static str,
}

impl  AppConfig {
    pub fn new() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),            
        }
    }
}