use actix_web::Responder;
use actix_web::{get, post, delete, put, web, HttpResponse};
use actix_web::HttpRequest;
use sqlx::MySqlPool;
use crate::models;

#[get("/vivienda/{id_enfermera}")]
pub async fn obtener_vivienda(db_pool: web::Data<MySqlPool>, req: HttpRequest) -> impl Responder {
    let result = models::get_vivienda(
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
#[post("/vivienda")]
pub async fn insertar_vivienda(db_pool: web::Data<MySqlPool>, new_vivienda: web::Json<models::Vivienda>) -> impl Responder {    
    let result = models::set_vivienda(db_pool.get_ref(), new_vivienda).await;
    match result {
        Ok(rows_affected) => HttpResponse::Ok().json(rows_affected),
        _ => HttpResponse::BadRequest().body("Sin Registros"),
    }
}
#[delete("/vivienda/{id_enfermera}")]
pub async fn eliminar_vivienda(db_pool: web::Data<MySqlPool>, req: HttpRequest) -> impl Responder {
    let result = models::delete_vivienda(
        db_pool.get_ref(),
 req
            .match_info()
            .get("id_enfermera")
            .unwrap()
    ).await;
    match result {
        Ok(rows_affected) => HttpResponse::Ok().json(rows_affected),
        _ => HttpResponse::BadRequest().body("Sin Registros"),
    }
}
#[put("/vivienda/{id_enfermera}")]
pub async fn actualizar_vivienda(db_pool: web::Data<MySqlPool>, req: HttpRequest, new_vivienda: web::Json<models::Vivienda>) -> impl Responder {
    let result = models::update_vivienda(
        db_pool.get_ref(),
        req
            .match_info()
            .get("id_enfermera")
            .unwrap(),  
            new_vivienda   
    ).await;
    match result {
        Ok(rows_affected) => HttpResponse::Ok().json(rows_affected),
        _ => HttpResponse::BadRequest().body("Sin Registros"),
    }
}
