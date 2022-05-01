use crate::tags::db;
use crate::tags::models::CreateTags;
use std::io;

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use io::ErrorKind::NotFound;

#[get("/")]
pub async fn tags(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::tags_list(&client).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[post("/")]
pub async fn add_tags(
    local_object: web::Json<CreateTags>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::tags_add(&client, local_object.clone()).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[get("/{id}")]
pub async fn get_tags(id_tags: web::Path<(i32,)>, db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::tags_id(&client, id_tags.0).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[delete("/{id}")]
pub async fn delete_tags(tags_id: web::Path<(i32,)>, db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::tags_delete(&client, tags_id.0).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[patch("/{id}")]
pub async fn update_tags(
    id_tags: web::Path<(i32,)>,
    local_object: web::Json<CreateTags>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::tags_update(&client, id_tags.0, local_object.clone()).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(tags);
    cfg.service(add_tags);
    cfg.service(update_tags);
    cfg.service(get_tags);
    cfg.service(delete_tags);
}

// #[delete("/{id}")]
// pub async fn delete_author(id_author: web::Path<(i32,)>,  db_pool: web::Data<Pool>) -> impl Responder {
//     let res = format!("{:?},", id_author.0);
//     println!("{:#?}", res);
//     res
// }
