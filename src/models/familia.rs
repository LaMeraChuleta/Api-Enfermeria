use serde::{ Serialize, Deserialize };
use actix_web::{Error, HttpRequest, HttpResponse, web, Responder, Result};
use futures::future::{ ready, Ready };
use sqlx::{ MySqlPool, FromRow, Row };
use futures::TryStreamExt;
use sqlx::Done;

#[derive(Serialize, FromRow, Deserialize)]
pub struct Familia {
    pub id_enfermera: String,
    pub id_familia: i32,
    pub nombres: String,
    pub apellido_m: String,
    pub apellido_p: String,
    pub parentesco: String,
    pub edad: i32,
}
impl Responder for Familia {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;
    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();        
        ready(Ok(
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body)
        ))
    }
}
pub async fn get_familia(pool: &MySqlPool, id_enfermera: &str) -> Result<Vec<Familia>> {
    let mut rows = sqlx::query(
        r#"
                SELECT * FROM familia
                    WHERE Id_Enfermera = ?        
            "#            
        )        
        .bind(id_enfermera)
        .fetch(pool);
    let mut vec_familia: Vec<Familia> = vec![];
    while let Some(row) = rows.try_next().await.unwrap() {
        vec_familia.push(Familia {
            id_enfermera: row.get("Id_Enfermera"),
            id_familia: row.get("Id_Familia"),
            nombres: row.get("Nombres"),
            apellido_m: row.get("Apellido_M"),
            apellido_p: row.get("Apellido_P"),
            parentesco: row.get("Parentesco"),     
            edad: row.get("Edad")   
        });                
    };            
    Ok(vec_familia)
}
pub async fn set_familia(pool: &MySqlPool, new_familia: web::Json<Familia>) -> Result<u64,> {
    let rows_affected = sqlx::query!(
        r#"
            INSERT INTO
            familia(Id_Enfermera,Nombres,Apellido_M,
                    Apellido_P,Parentesco,Edad)
            VALUE (?,?,?,?,?,?)
        "#,
        &new_familia.id_enfermera.to_string(),        
        &new_familia.nombres.to_string(),
        &new_familia.apellido_m.to_string(),
        &new_familia.apellido_p.to_string(),
        &new_familia.parentesco.to_string(),
        &new_familia.edad,  
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();    
    Ok(rows_affected)
}
pub async fn delete_familiar(pool: &MySqlPool, id_enfermera: &str, id_familia: i32) -> Result<u64,> {
    let rows_affected = sqlx::query!(
        r#"
            DELETE FROM
                familia
            WHERE Id_Enfermera = ? AND Id_Familia = ?            
        "#,
        id_enfermera,
        id_familia
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();
    Ok(rows_affected)
}
pub async fn update_familiar(pool: &MySqlPool, id_enfermera: &str, id_familiar: i32, new_familiar: web::Json<Familia>) -> Result<u64,> {
    let rows_affected = sqlx::query!(
        r#"
            UPDATE familia
                SET Id_Enfermera = ?, Id_Familia = ?, Nombres = ?, 
                Apellido_M = ?, Apellido_P = ?, Parentesco = ?, Edad = ?                                                
            WHERE Id_Enfermera = ? AND Id_Familia = ?
        "#,
        &new_familiar.id_enfermera.to_string(),
        &new_familiar.id_familia.to_string(),
        &new_familiar.nombres.to_string(),
        &new_familiar.apellido_m.to_string(),
        &new_familiar.apellido_p.to_string(),
        &new_familiar.parentesco.to_string(),
        &new_familiar.edad,
        id_enfermera,
        id_familiar,            
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();
    Ok(rows_affected)
}
