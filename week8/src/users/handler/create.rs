use actix_web::{post, web, HttpResponse ,Responder};
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use crate::users::{handler::response::ApiResponse, storage::UserStorage, user::User};

#[post("/")]
pub async fn create_user(storage: web::Data<Arc<Mutex<UserStorage>>>, user: web::Json<User>) -> impl Responder {
    let mut user = user.into_inner();
    user.id = Some(Uuid::now_v7().to_string());

    let mut storage = match storage.lock() {
        Ok(s) => s,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(String::from("Storage lock error")));
        }
    };

    storage.create(user.clone());

    HttpResponse::Ok().json(ApiResponse::success(user))
}
