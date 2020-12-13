use serde::{ Serialize, Deserialize };
use actix_web::{Error, HttpRequest, HttpResponse, web, Responder, Result};
use futures::future::{ ready, Ready };
use sqlx::{ MySqlPool, FromRow, Row };
use futures::TryStreamExt;
use sqlx::Done;
#[derive(Serialize, FromRow, Deserialize)]
pub struct Estudio {
    pub id_enfermera: String,
    pub id_estudio: i32,
    pub nivel: String,
    pub titulo: String,
    pub institucion: String,
    pub tipo_escuela: String    
}

impl Responder for Estudio {
    type Error = Error;
    type Future = Ready<Result<HttpResponse,Error>>;
    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body)
        ))
    }
}
pub async fn get_estudios(pool: &MySqlPool, id_enfermera: &str) -> Result<Vec<Estudio>> {
    let mut rows = sqlx::query(
        r#"
                SELECT * FROM estudios
                    WHERE Id_Enfermera = ?        
            "#            
        )        
        .bind(id_enfermera)
        .fetch(pool);
    let mut vec_familia: Vec<Estudio> = vec![];
    while let Some(row) = rows.try_next().await.unwrap() {
        vec_familia.push(Estudio {
            id_enfermera: row.get("Id_Enfermera"),
            id_estudio: row.get("Id_Estudio"),
            nivel: row.get("Nivel"),
            titulo: row.get("Titulo"),
            institucion: row.get("Institucion"),
            tipo_escuela: row.get("Tipo_Escuela"),            
        });                
    };            
    Ok(vec_familia)
}
pub async fn set_estudio(pool: &MySqlPool, new_estudio: web::Json<Estudio>) -> Result<u64,> {
    let rows_affected = sqlx::query!(
        r#"
            INSERT INTO
            estudios(Id_Enfermera,Nivel,Titulo,
                    Institucion,Tipo_Escuela)
            VALUE (?,?,?,?,?)
        "#,
        &new_estudio.id_enfermera.to_string(),        
        &new_estudio.nivel.to_string(),
        &new_estudio.titulo.to_string(),
        &new_estudio.institucion.to_string(),
        &new_estudio.tipo_escuela.to_string(),    
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();    
    Ok(rows_affected)
}
pub async fn delete_estudio(pool: &MySqlPool, id_enfermera: &str, id_estudio: i32) -> Result<u64,> {
    let rows_affected = sqlx::query!(
        r#"
            DELETE FROM
                estudios
            WHERE Id_Enfermera = ? AND Id_Estudios = ?            
        "#,
        id_enfermera,
        id_estudio
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();
    Ok(rows_affected)
}
pub async fn update_estudio(pool: &MySqlPool, id_enfermera: &str, id_estudios: i32, new_estudio: web::Json<Estudio>) -> Result<u64,> {
    let rows_affected = sqlx::query!(
        r#"
            UPDATE estudios
                SET Id_Enfermera = ?, Nivel = ?, 
                Titulo = ?, Institucion = ?, Tipo_Escuela = ?
            WHERE Id_Enfermera = ? AND Id_Estudios = ?
        "#,
        &new_estudio.id_enfermera.to_string(),        
        &new_estudio.nivel.to_string(),
        &new_estudio.titulo.to_string(),
        &new_estudio.institucion.to_string(),
        &new_estudio.tipo_escuela.to_string(),        
        id_enfermera,
        id_estudios,            
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();
    Ok(rows_affected)
}
