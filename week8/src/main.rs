mod users;

use std::sync::{Arc, Mutex};

use actix_web::{App, HttpServer, web};
use users::handler::create_user;
use users::handler::get_user;
use users::handler::fetch_user;

use crate::users::handler::delete_user;
use crate::users::handler::update_user;
use crate::users::{storage::UserStorage};

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let storage = UserStorage::new();
    let storage = Arc::new(Mutex::new(storage));
    let storage  = web::Data::new(storage);

    HttpServer::new(move || {
        App::new()
            .app_data(storage.clone())
            .service(
                web::scope("/users")
                .service(create_user)
                .service(get_user)
                .service(fetch_user)
                .service(update_user)
                .service(delete_user)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
