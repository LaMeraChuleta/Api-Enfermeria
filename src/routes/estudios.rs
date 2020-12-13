use actix_web::Responder;
use actix_web::{get, post, delete, put, web, HttpResponse};
use actix_web::HttpRequest;
use sqlx::MySqlPool;
use crate::models;

#[get("/estudios/{id_enfermera}")]
pub async fn obtener_estudios(db_pool: web::Data<MySqlPool>, req: HttpRequest) -> impl Responder {
    let result = models::get_estudios(
        db_pool.get_ref(),
        req
            .match_info()
            .get("id_enfermera")
            .unwrap() 
    ).await;
    match result {
        Ok(estudios) => HttpResponse::Ok().json(estudios),
        _ => HttpResponse::BadRequest().body("Sin Registros"),
    }
}
#[post("/estudios")]
pub async fn insertar_estudio(db_pool: web::Data<MySqlPool>, new_estudio: web::Json<models::Estudio>) -> impl Responder {    
    let result = models::set_estudio(db_pool.get_ref(), new_estudio).await;
    match result {
        Ok(rows_affected) => HttpResponse::Ok().json(rows_affected),
        _ => HttpResponse::BadRequest().body("Sin Registros"),
    }
}
#[delete("/estudios/{id_enfermera}/{id_estudio}")]
pub async fn eliminar_estudio(db_pool: web::Data<MySqlPool>, req: HttpRequest) -> impl Responder {
    let result = models::delete_estudio(
        db_pool.get_ref(),
 req
            .match_info()
            .get("id_enfermera")
            .unwrap(),
    req
            .match_info()
            .get("id_estudio")
            .unwrap()
            .parse::<i32>()
            .unwrap(),    
    ).await;
    match result {
        Ok(rows_affected) => HttpResponse::Ok().json(rows_affected),
        _ => HttpResponse::BadRequest().body("Sin Registros"),
    }
}
#[put("/estudios/{id_enfermera}/{id_estudio}")]
pub async fn actualizar_estudio(db_pool: web::Data<MySqlPool>, req: HttpRequest, new_estudio: web::Json<models::Estudio>) -> impl Responder {
    let result = models::update_estudio(
        db_pool.get_ref(),
        req
            .match_info()
            .get("id_enfermera")
            .unwrap(),
            req
            .match_info()
            .get("id_estudio")
            .unwrap()
            .parse::<i32>()
            .unwrap(),   
            new_estudio   
    ).await;
    match result {
        Ok(rows_affected) => HttpResponse::Ok().json(rows_affected),
        _ => HttpResponse::BadRequest().body("Sin Registros"),
    }
}
