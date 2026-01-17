use std::sync::{Arc, Mutex};


use actix_web::{HttpResponse, Responder, get, web};

use crate::users::handler::response::ApiResponse;
use crate::users::storage::{UserStorage};

#[get("/")]
pub async  fn fetch_user(
    storage: web::Data<Arc<Mutex<UserStorage>>>
) -> impl Responder {
    let storage = match storage.lock() {
        Ok(s) => s,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(String::from("Storage lock error")));
        }
    };

    let users = storage.fetch();

    HttpResponse::Ok().json(ApiResponse::success(users))
}
