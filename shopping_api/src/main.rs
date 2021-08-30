#[macro_use]
extern crate diesel;
extern crate chrono;

use actix_web::{middleware, App, HttpServer};
use db::*;
use orders::routes;

mod db;
mod manage_users;
mod orders;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Establish DB Pool
    let pool = establish_db_pool();
    // Read host usr from .env file
    let host_url = std::env::var("HOST_URL").expect("HOST_URL must be set.");

    println!("Starting server at: {}", &host_url);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(routes::get_products)
            .service(routes::place_order)
            .service(routes::get_orders)
            .service(routes::register_user)
            .service(routes::add_products_in_stock)
            .service(routes::show_users)
    })
    .bind(&host_url)?
    .run()
    .await
}
