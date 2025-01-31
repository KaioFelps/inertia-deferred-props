use actix_session::SessionMiddleware;
use actix_web::{
    cookie::{Key, SameSite},
    web::Data,
    App, HttpServer,
};
use file_session::FileSessionStore;
use helpers::{get_env_mode, RustEnv};
use inertia_rust::actix::InertiaMiddleware;
use middlewares::{
    garbage_collector::GarbageCollectorMiddleware, inertia_reflasher::ReflashTemporarySession,
};
use std::io;

mod entities;
mod file_session;
mod helpers;
mod inertia;
mod middlewares;
mod routes;
mod vite;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let vite = vite::initialize_vite().await;
    let inertia = inertia::initialize_inertia(vite).await?;
    let inertia = Data::new(inertia);

    // obviously this should be an actual key and be safely stored inside your environment variables
    // instead of a hard-coded string slice
    let key = Key::derive_from("voiJKNMNJvbjknjHBO86&&$(#IUKJS@1".as_bytes());
    let storage = FileSessionStore::default();

    HttpServer::new(move || {
        App::new()
            .app_data(inertia.clone())
            .wrap(GarbageCollectorMiddleware)
            .wrap(InertiaMiddleware::new())
            .wrap(ReflashTemporarySession)
            .wrap(
                SessionMiddleware::builder(storage.clone(), key.clone())
                    .cookie_domain(Some("localhost".into()))
                    .cookie_http_only(true)
                    .cookie_same_site(SameSite::Strict)
                    .cookie_name("test_cookie_session_id".into())
                    .cookie_secure(get_env_mode() == RustEnv::Production)
                    .build(),
            )
            .configure(routes::register)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
