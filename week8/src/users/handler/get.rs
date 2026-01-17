use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, Responder, get, web};

use crate::users::{handler::response::ApiResponse, storage::{UserStorage}};

#[get("/{id}")]
pub async fn get_user(
    storage: web::Data<Arc<Mutex<UserStorage>>>,
    id: web::Path<String>,
) -> impl Responder {
    let id = id.into_inner();

    let storage = match storage.lock() {
        Ok(s) => s,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(String::from("Storage lock error")));
        }
    };

    match storage.get_by_id(&id) {
        Some(user) => {
            HttpResponse::Ok().json(ApiResponse::success(user))
        }
        None => {
            HttpResponse::NotFound().json(
                ApiResponse::<()>::error(String::from("user not found"))
            )
        }
    }
}
