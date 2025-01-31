use std::env;

pub fn get_env_mode() -> RustEnv {
    let rust_env = env::var("RUST_ENV").unwrap_or("production".into());
    match rust_env.as_str() {
        "production" => RustEnv::Production,
        _ => RustEnv::Develpoment,
    }
}

#[derive(PartialEq)]
pub enum RustEnv {
    Production,
    Develpoment,
}
