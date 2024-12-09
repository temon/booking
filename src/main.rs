mod services;
mod models;
mod routes;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use crate::routes::booking_route::{cancel_booking, create_booking, get_bookings};
use crate::routes::dog_route::create_dog;
use crate::routes::owner_route::create_owner;
use crate::services::db::Database;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Google nya mas najib, masukin apa yang mau kamu cari disini:")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move|| App::new().app_data(db_data.clone())
        .service(hello)
        .service(create_dog)
        .service(create_booking)
        .service(create_owner)
        .service(get_bookings)
        .service(cancel_booking)
    )
        .bind(("localhost", 5001))
        ?.run()
        .await
}