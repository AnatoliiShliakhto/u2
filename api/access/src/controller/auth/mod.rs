mod authorize;
mod revoke;
mod token;

pub use self::{
    authorize::*,
    revoke::*,
    token::*,
};

static COOKIE_JWT: &str = "JWT_RT";