use actix_web::Responder;
use actix_web::{get, post, delete, put, web, HttpResponse};
use actix_web::HttpRequest;
use sqlx::MySqlPool;
use crate::models;


#[get("/familia/{id_enfermera}")]
pub async fn obtener_familia(db_pool: web::Data<MySqlPool>, req: HttpRequest) -> impl Responder {
    let result = models::get_familia(
        db_pool.get_ref(),
        req
            .match_info()
            .get("id_enfermera")
            .unwrap() 
    ).await;
    match result {
        Ok(familiares) => HttpResponse::Ok().json(familiares),
        _ => HttpResponse::BadRequest().body("Sin Registros"),
    }
}
#[post("/familia")]
pub async fn insertar_familiar(db_pool: web::Data<MySqlPool>, new_familia: web::Json<models::Familia>) -> impl Responder {    
    let result = models::set_familia(db_pool.get_ref(), new_familia).await;
    match result {
        Ok(rows_affected) => HttpResponse::Ok().json(rows_affected),
        _ => HttpResponse::BadRequest().body("Sin Registros"),
    }
}
#[delete("/familia/{id_enfermera}/{id_familia}")]
pub async fn eliminar_familiar(db_pool: web::Data<MySqlPool>, req: HttpRequest) -> impl Responder {
    let result = models::delete_familiar(
        db_pool.get_ref(),
 req
            .match_info()
            .get("id_enfermera")
            .unwrap(),
    req
            .match_info()
            .get("id_familia")
            .unwrap()
            .parse::<i32>()
            .unwrap(),    
    ).await;
    match result {
        Ok(rows_affected) => HttpResponse::Ok().json(rows_affected),
        _ => HttpResponse::BadRequest().body("Sin Registros"),
    }
}
#[put("/familia/{id_enfermera}/{id_familia}")]
pub async fn actualizar_familiar(db_pool: web::Data<MySqlPool>, req: HttpRequest, new_enfermera: web::Json<models::Familia>) -> impl Responder {
    let result = models::update_familiar(
        db_pool.get_ref(),
        req
            .match_info()
            .get("id_enfermera")
            .unwrap(),
            req
            .match_info()
            .get("id_familia")
            .unwrap()
            .parse::<i32>()
            .unwrap(),   
        new_enfermera   
    ).await;
    match result {
        Ok(rows_affected) => HttpResponse::Ok().json(rows_affected),
        _ => HttpResponse::BadRequest().body("Sin Registros"),
    }
}
