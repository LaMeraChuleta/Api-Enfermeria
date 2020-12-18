use actix_web::Responder;
use actix_web::{get, post, delete, put, web, HttpResponse};
use actix_web::HttpRequest;
use sqlx::MySqlPool;
use crate::models;


#[get("/pdf/{matricula}")]
pub async fn obtener_pdf_enfermera(db_pool: web::Data<MySqlPool>, req: HttpRequest) -> impl Responder {
    let result = models::imprimir_pdf_enfermera(
        db_pool.get_ref(), 
        req.match_info().get("matricula").unwrap()).await;
    std::thread::sleep(std::time::Duration::from_secs(2));
    HttpResponse::Ok().json(result)    
}
#[get("/catalogo/tipoEnfermera")]
pub async fn obtener_catalogo_tipo_enfermera(db_pool: web::Data<MySqlPool>) -> impl Responder {
    let result = models::get_catalogo_tipo_enfermera(db_pool.get_ref()).await;    
    match result {
        Ok(catalogo) => HttpResponse::Ok().json(catalogo),
        _ => HttpResponse::BadRequest().body("Sin Registros"),
    }
}
#[get("/enfermera")]
pub async fn obtener_enfermeras(db_pool: web::Data<MySqlPool>) -> impl Responder {
    let result = models::get_enfermeras(db_pool.get_ref()).await;
    match result {
        Ok(enfermera) => HttpResponse::Ok().json(enfermera),
        _ => HttpResponse::BadRequest().body("Sin Registros"),
    }
}
#[post("/enfermera")]
pub async fn insertar_enfermera(db_pool: web::Data<MySqlPool>, new_enfermera: web::Json<models::Enfermera>) -> impl Responder {
    let result = models::set_enfermeras(db_pool.get_ref(), new_enfermera).await;
    match result {
        Ok(rows_affected) => HttpResponse::Ok().json(rows_affected),
        _ => HttpResponse::BadRequest().body("Sin Registros"),
    }
}
#[delete("/enfermera/{matricula}")]
pub async fn eliminar_enfermera(db_pool: web::Data<MySqlPool>, req: HttpRequest) -> impl Responder {    
    let result = models::delete_enfermera(
        db_pool.get_ref(),
    req
            .match_info()
            .get("matricula")
            .unwrap()   
    ).await;
    match result {
        Ok(rows_affected) => HttpResponse::Ok().json(rows_affected),
        _ => HttpResponse::BadRequest().body("Sin Registros"),
    }
}
#[put("/enfermera/{matricula}")]
pub async fn actualizar_enfermera(db_pool: web::Data<MySqlPool>, req: HttpRequest, new_enfermera: web::Json<models::Enfermera>) -> impl Responder {
    let result = models::update_enfermera(
        db_pool.get_ref(),
        req
            .match_info()
            .get("matricula")
            .unwrap(),
        new_enfermera   
    ).await;
    match result {
        Ok(rows_affected) => HttpResponse::Ok().json(rows_affected),
        _ => HttpResponse::BadRequest().body("Sin Registros"),
    }
}
