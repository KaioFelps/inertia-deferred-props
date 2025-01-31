use vite_rust::{Vite, ViteConfig};

pub async fn initialize_vite() -> Vite {
    match Vite::new(ViteConfig::new(
        "public/assets/manifest.json",
        vec!["www/app.tsx"],
    ))
    .await
    {
        Err(err) => panic!("{err}"),
        Ok(vite) => vite,
    }
}
