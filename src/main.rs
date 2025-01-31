use actix_web::{web::Data, App, HttpServer};
use std::io;

mod inertia;
mod vite;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let vite = vite::initialize_vite().await;
    let inertia = inertia::initialize_inertia(vite).await?;
    let inertia = Data::new(inertia);

    HttpServer::new(move || App::new().app_data(inertia.clone()))
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}
