use inertia_rust::{template_resolvers::ViteHBSTemplateResolver, InertiaConfig};
use inertia_rust::{Inertia, InertiaVersion};
use std::io;
use vite_rust::Vite;

use crate::helpers::{get_env_mode, RustEnv};

pub async fn initialize_inertia(vite: Vite) -> io::Result<Inertia> {
    let version = vite.get_hash().unwrap_or("development-assets").to_string();
    let dev_mode = get_env_mode() != RustEnv::Production;

    let template_resolver = ViteHBSTemplateResolver::new(vite, "www/root.hbs", dev_mode)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err.get_cause()))?;

    Inertia::new(
        InertiaConfig::builder()
            .set_template_resolver(Box::new(template_resolver))
            .set_version(InertiaVersion::Literal(version))
            .set_url("localhost:3000")
            .build(),
    )
}
