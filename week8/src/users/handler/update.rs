use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, Responder, put, web};

use crate::users::{handler::response::ApiResponse, storage::{UserStorage}, user::User};

#[put("/{id}")]
pub async fn update_user(
    storage: web::Data<Arc<Mutex<UserStorage>>>,
    id: web::Path<String>,
    user: web::Json<User>,
) -> impl Responder {
    let id = id.into_inner();
    let user = user.into_inner();

    let mut storage = match storage.lock() {
        Ok(s) => s,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(String::from("Storage lock error")));
        }
    };

    match storage.update(&id, user) {
        Some(updated_user) => {
            HttpResponse::Ok().json(ApiResponse::success(updated_user))
        }
        None => {
            HttpResponse::NotFound().json(
                ApiResponse::<()>::error(String::from("user not found"))
            )
        }
    }
}
