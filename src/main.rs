use actix_session::SessionMiddleware;
use actix_web::{
    cookie::{Key, SameSite},
    web::Data,
    App, HttpServer,
};
use file_session::FileSessionStore;
use helpers::{get_client_template_engine, get_env_mode, ClientTemplateEngine, RustEnv};
use inertia_rust::{actix::InertiaMiddleware, hashmap, InertiaProp};
use middlewares::{
    garbage_collector::GarbageCollectorMiddleware, inertia_reflasher::ReflashTemporarySession,
};
use std::{io, sync::Arc};

mod entities;
mod file_session;
mod helpers;
mod inertia;
mod middlewares;
mod routes;
mod vite;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenvy::dotenv().ok();

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
            .wrap(InertiaMiddleware::new().with_shared_props(Arc::new(|_| {
                let client = get_client_template_engine();

                Box::pin(async move {
                    hashmap![
                        "client" => InertiaProp::always(client.to_string()),
                        "using_react" => InertiaProp::always(client == ClientTemplateEngine::React)
                    ]
                })
            })))
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
            .service(actix_files::Files::new("/", "./public/").prefer_utf8(true))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
