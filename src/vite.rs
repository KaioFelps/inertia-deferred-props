use vite_rust::{Vite, ViteConfig};

use crate::helpers::{get_client_template_engine, ClientTemplateEngine};

pub async fn initialize_vite() -> Vite {
    let entrypoint = match get_client_template_engine() {
        ClientTemplateEngine::React => "www/react-app.tsx",
        ClientTemplateEngine::Svelte => "www/svelte-app.ts",
    };

    match Vite::new(
        ViteConfig::new("public/build/manifest.json", vec![entrypoint]).set_prefix("build"),
    )
    .await
    {
        Err(err) => panic!("{err}"),
        Ok(vite) => vite,
    }
}
