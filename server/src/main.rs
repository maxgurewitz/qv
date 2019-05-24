#[macro_use]
extern crate serde_derive;
extern crate actix_web;
use actix_web::{http, server, App, Error, HttpRequest, Json, Responder};

#[derive(Serialize)]
struct HomePageResource {
    name: String,
}

fn homepageRoute(req: HttpRequest) -> Result<Json<HomePageResource>, Error> {
    Ok(Json(HomePageResource {
        name: "foo".to_string(),
    }))
}

fn main() {
    server::new(|| {
        App::new().resource("/api/home", |r| {
            r.method(http::Method::POST).with(homepageRoute)
        })
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}
