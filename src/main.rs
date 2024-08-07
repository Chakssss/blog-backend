mod models;
mod controller;
mod db;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use controller::{signup,login};
use crate::controller::{add_blog, get_blogs, get_user_by_email};
use crate::db::get_database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db=get_database().await;
    println!("Server Running");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(cors)
            .service(login)
            .service(signup)
            .service(add_blog)
            .service(get_blogs)
            .service(get_user_by_email)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
