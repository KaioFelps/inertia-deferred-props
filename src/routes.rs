use std::time::Duration;

use actix_web::{get, post, web::ServiceConfig, HttpRequest, Responder};
use inertia_rust::{
    hashmap, prop_resolver, Inertia, InertiaFacade, InertiaProp, IntoInertiaPropResult,
};
use tokio::time::sleep;

use crate::entities::user::User;

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(index)
        .service(built_in_deferred)
        .service(custom_deferred)
        .service(ping_pong);
}

#[get("/built-in")]
async fn built_in_deferred(req: HttpRequest) -> impl Responder {
    Inertia::render_with_props(
        &req,
        "built-in-deferred".into(),
        hashmap![
            "users" => InertiaProp::defer(prop_resolver!({
                sleep(Duration::from_secs(1)).await;

                vec![
                    User { id: 1, name: "John Doe" },
                    User { id: 2, name: "Foo Guy" },
                    User {id: 3, name: "Alice"}
                ].into_inertia_value()
            }))
        ],
    )
    .await
}

#[get("/custom")]
async fn custom_deferred(req: HttpRequest) -> impl Responder {
    Inertia::render_with_props(
        &req,
        "custom-deferred".into(),
        hashmap![
            "users" => InertiaProp::defer(prop_resolver!({
                sleep(Duration::from_secs(1)).await;

                vec![
                    User { id: 1, name: "John Doe" },
                    User { id: 2, name: "Foo Guy" },
                    User {id: 3, name: "Alice"}
                ].into_inertia_value()
            }))
        ],
    )
    .await
}

#[post("/pingpong")]
async fn ping_pong(req: HttpRequest) -> impl Responder {
    Inertia::back(&req)
}

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    Inertia::render(&req, "index".into()).await
}
