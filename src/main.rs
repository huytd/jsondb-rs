#[macro_use]
extern crate diesel;
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv;
use actix_web::{HttpServer, App, web, HttpResponse, middleware};
use actix_files as fs;
use crate::model::{Store, NewStore};

mod model;
mod schema;

use self::schema::stores::dsl::*;
use serde_json::Value;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn create_store(request_data: web::Json<serde_json::Value>, pool: web::Data<Pool>) -> HttpResponse {
    let serialized = request_data.to_string();
    let uuid = format!("{}", uuid::Uuid::new_v4());
    let new_entry = NewStore {
        data: &serialized,
        api_id: &uuid
    };
    let conn = &pool.get().unwrap();
    if let Ok(_) = diesel::insert_into(stores).values(&new_entry).execute(conn) {
        if let Ok(mut result) = stores.filter(api_id.eq(&uuid)).load::<model::Store>(conn) {
            return HttpResponse::Ok().json::<Value>(result.pop().unwrap().into())
        }
    }
    HttpResponse::InternalServerError().into()
}

async fn read_store(api_key: web::Path<String>, pool: web::Data<Pool>) -> HttpResponse {
    let conn = &pool.get().unwrap();
    if let Ok(mut result) = stores.filter(api_id.eq(api_key.into_inner())).load::<model::Store>(conn) {
        let entry: Store = result.pop().unwrap();
        return HttpResponse::Ok().json::<Value>(entry.into())
    }
    HttpResponse::InternalServerError().into()
}

async fn update_store(api_key: web::Path<String>, request_data: web::Json<serde_json::Value>, pool: web::Data<Pool>) -> HttpResponse {
    let conn = &pool.get().unwrap();
    let candidates = stores.filter(api_id.eq(api_key.into_inner()));
    if let Ok(result) = diesel::update(candidates).set(data.eq(request_data.to_string())).get_result::<model::Store>(conn) {
        return HttpResponse::Ok().json::<Value>(result.into())
    }
    HttpResponse::InternalServerError().into()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();
    let port = std::env::var("PORT").unwrap_or("3123".to_string()).parse::<u16>().unwrap();
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/storage").route(web::post().to(create_store)))
            .service(web::resource("/storage/{id}")
                .route(web::get().to(read_store))
                .route(web::put().to(update_store))
            )
            .default_service(fs::Files::new("", "./public"))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
