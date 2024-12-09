use std::convert::TryFrom;
use actix_web::{get, post, put, HttpResponse};
use actix_web::web::{Data, Json, Path};
use crate::models::booking_model::{Booking, BookingRequest};
use crate::services::db::Database;

#[post("/booking")]
pub async fn create_booking(db: Data<Database>, request: Json<BookingRequest>) -> HttpResponse {
    match db
        .create_booking(
            Booking::try_from(BookingRequest{
                owner: request.owner.clone(),
                start_time: request.start_time.clone(),
                duration_in_minutes: request.duration_in_minutes.clone()
            })
                .expect("Error converting BookingRequest to Booking.")
        ).await
    {
        Ok(booking) => HttpResponse::Ok().json(booking),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        _ => HttpResponse::InternalServerError().body("Can not create booking.")
    }
}

#[get("/bookings")]
pub async fn get_bookings(db: Data<Database>) -> HttpResponse {
    match db.get_bookings().await {
        Ok(booking) => {
            HttpResponse::Ok().json(booking)
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

#[put("/booking/{id}/cancel")]
pub async fn cancel_booking(db: Data<Database>, path: Path<(String,)>) -> HttpResponse {
    let id = path.into_inner().0;
    match db.cancel_booking(id.as_str()).await {
        Ok(booking) => {
            HttpResponse::Ok().json(booking)
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}