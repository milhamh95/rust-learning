use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, Responder, delete, web};

use crate::users::{handler::response::ApiResponse, storage::UserStorage};

#[delete("/{id}")]
pub async fn delete_user (
    storage: web::Data<Arc<Mutex<UserStorage>>>,
    id: web::Path<String>,
) -> impl Responder {
    let id = id.into_inner();

    let mut storage = match storage.lock() {
        Ok(s) => s,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(String::from("Storage lock error")));
        }
    };

    match storage.delete(&id) {
        Some(deleted_user) => {
            HttpResponse::Ok().json(ApiResponse::success(deleted_user))
        }
        None => {
            HttpResponse::NotFound().json(
                ApiResponse::<()>::error(String::from("user not found"))
            )
        }
    }
}
