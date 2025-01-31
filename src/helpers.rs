use std::{env, fmt::Display};

pub fn get_env_mode() -> RustEnv {
    let rust_env = env::var("RUST_ENV").unwrap_or("production".into());
    match rust_env.as_str() {
        "production" => RustEnv::Production,
        _ => RustEnv::Develpoment,
    }
}

pub fn get_client_template_engine() -> ClientTemplateEngine {
    let client = env::var("CLIENT").unwrap_or("react".into());
    match client.as_str() {
        "svelte" => ClientTemplateEngine::Svelte,
        _ => ClientTemplateEngine::React,
    }
}

#[derive(PartialEq)]
pub enum RustEnv {
    Production,
    Develpoment,
}

#[derive(PartialEq)]
pub enum ClientTemplateEngine {
    React,
    Svelte,
}

impl Display for ClientTemplateEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ClientTemplateEngine::React => "react",
                ClientTemplateEngine::Svelte => "svelte",
            }
        )
    }
}
