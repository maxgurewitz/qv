#[macro_use]
extern crate serde_derive;
extern crate actix_web;
use actix_web::{http, server, App, Error, HttpRequest, HttpResponse, Json};

#[derive(Serialize)]
struct HomePageResource {
    name: String,
}

fn home_page_route(_req: HttpRequest) -> Result<Json<HomePageResource>, Error> {
    Ok(Json(HomePageResource {
        name: "foo".to_string(),
    }))
}

fn not_found_page_route(_req: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NotFound()
        .content_type("application/json")
        .body("{ \"message\": \"route not found\" }"))
}

// auth flow https://auth0.com/docs/flows/concepts/auth-code
fn main() {
    server::new(|| {
        App::new()
            .scope("/api", |scope| {
                scope.route("/home", http::Method::POST, home_page_route)
            })
            .default_resource(|r| r.with(not_found_page_route))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}
