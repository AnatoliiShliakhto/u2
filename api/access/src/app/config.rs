use crate::middleware::JwtKeys;
use ::api_util::env;

pub struct AppConfig {
    pub name: &'static str,
    pub version: &'static str,
    pub security: Security,
}

pub struct Security {
    pub jwt: Jwt,
    pub jwt_keys: JwtKeys,
    pub delete_expired_tokens_interval: u64,
    pub set_cookie: &'static str,
}

pub struct Jwt {
    pub secret: &'static str,
    pub issuer: &'static str,
    pub subject: &'static str,
    pub access_expires_in: i64,
    pub refresh_expires_in: i64,
}

impl AppConfig {
    pub fn new() -> Self {
        let jwt = Jwt {
            secret: env::get_var_or_default("JWT_SECRET", "secret"),
            issuer: env::get_var_or_default("JWT_ISSUER", ""),
            subject: env::get_var_or_default("JWT_SUBJECT", ""),
            access_expires_in: env::get_var_or_default("JWT_ACCESS_EXPIRATION", "600")
                .parse()
                .unwrap_or(600),
            refresh_expires_in: env::get_var_or_default("JWT_REFRESH_EXPIRATION", "1296000")
                .parse()
                .unwrap_or(1_296_000),
        };

        let jwt_keys = JwtKeys::new(jwt.secret.as_bytes());

        let security = Security {
            jwt,
            jwt_keys,
            delete_expired_tokens_interval: env::get_var_or_default("JWT_DELETE_INTERVAL", "1800")
                .parse()
                .unwrap_or(1800),
            set_cookie: env::get_var_or_default(
                "JWT_SET_COOKIE",
                "SameSite=Strict; HttpOnly; Secure;",
            ),
        };

        Self {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            security,
        }
    }
}
