use serde::{Serialize, Deserialize};

use std::collections::HashMap;
use actix_web::{error, middleware, get, post, web, http::header, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use tera::Tera;

use crate::persist;
use crate:: entity;

fn list_posts() -> Vec<entity::Post> {
    let posts = persist::fetch_posts().unwrap();
    posts.iter()
        .map(|p|
            entity::Post { id: p.0, title: p.1.clone(), text: p.2.clone(), timestamp: p.3.clone()} )
        .collect::<Vec<entity::Post>>()
}

#[get("/new-post")]
async fn new_post(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    tmpl.render("new_post.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))
        .map(|page_text| HttpResponse::Ok().content_type("text/html").body(page_text))
}

#[get("/")]
async fn all_posts(tmpl: web::Data<tera::Tera>, query: web::Query<HashMap<String, String>>) -> Result<HttpResponse, Error> {
    let posts = list_posts();
    let mut ctx = tera::Context::new();
    ctx.insert("posts", &posts);
    tmpl.render("posts.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))
        .map(|page_text| HttpResponse::Ok().content_type("text/html").body(page_text))
}

#[derive(Serialize, Deserialize, Debug)]
struct CreatePostRequest {
    title: String,
    post_text: String
}

#[post("/create-post")]
async fn crate_post(payload: web::Form<CreatePostRequest>) -> impl Responder {
    persist::insert_post(&payload.title, &payload.post_text);

    HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish()
}

pub async fn create_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        App::new()
            .data(tera)
            .wrap(middleware::Logger::default())
            .service(all_posts)
            .service(new_post)
            .service(crate_post)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}