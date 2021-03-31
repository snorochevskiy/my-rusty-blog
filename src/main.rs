#[macro_use]

extern crate serde;

mod persist;
mod entity;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::create_server()
        .await
}