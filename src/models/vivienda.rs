use serde::{ Serialize, Deserialize };
use actix_web::{Error, HttpRequest, HttpResponse, web, Responder, Result};
use futures::future::{ ready, Ready };
use sqlx::{ MySqlPool, FromRow, Row };
use futures::TryStreamExt;
use sqlx::Done;
#[derive(Serialize, FromRow, Deserialize)]
pub struct Vivienda {
    pub id_enfermera: String,
    pub estado: String,
    pub delegacion: String,
    pub colonia: String,
    pub calle: String,
    pub cp: String, 
    pub num_ext: i32,    
    pub num_int: i32,    
}

impl Responder for Vivienda {
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
pub async fn get_vivienda(pool: &MySqlPool, id_enfermera: &str) -> Result<Vivienda> {
    let mut rows = sqlx::query(
        r#"
                SELECT * FROM vivienda
                    WHERE Id_Enfermera = ?        
            "#            
        )        
        .bind(id_enfermera)
        .fetch(pool);    
    let row = rows.try_next().await.unwrap().unwrap();
    let vivienda = Vivienda {
        id_enfermera: row.get("Id_Enfermera"),
        estado: row.get("Estado"),
        delegacion: row.get("Delegacion"),
        colonia: row.get("Colonia"),
        calle: row.get("Calle"),
        cp: row.get("CP"),
        num_ext: row.get("Num_Ext"),
        num_int: row.get("Num_Int"),  
    };                             
    Ok(vivienda)
}
pub async fn set_vivienda(pool: &MySqlPool, new_vivienda: web::Json<Vivienda>) -> Result<u64,> {
    let rows_affected = sqlx::query!(
        r#"
            INSERT INTO
            vivienda(Id_Enfermera,Estado,Delegacion,
                    Colonia,Calle,CP,Num_Ext,Num_Int)
            VALUE (?,?,?,?,?,?,?,?)
        "#,
        &new_vivienda.id_enfermera.to_string(),        
        &new_vivienda.estado.to_string(),
        &new_vivienda.delegacion.to_string(),
        &new_vivienda.colonia.to_string(),
        &new_vivienda.calle.to_string(),    
        &new_vivienda.cp.to_string(),    
        &new_vivienda.num_ext.to_string(),    
        &new_vivienda.num_int.to_string(),    
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();    
    Ok(rows_affected)
}
pub async fn delete_vivienda(pool: &MySqlPool, id_enfermera: &str) -> Result<u64,> {
    let rows_affected = sqlx::query!(
        r#"
            DELETE FROM
                vivienda
            WHERE Id_Enfermera = ?            
        "#,
        id_enfermera,
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();
    Ok(rows_affected)
}
pub async fn update_vivienda(pool: &MySqlPool, id_enfermera: &str, new_vivienda: web::Json<Vivienda>) -> Result<u64,> {
    let rows_affected = sqlx::query!(
        r#"
            UPDATE vivienda
                SET Id_Enfermera = ?, Estado = ?, Delegacion = ?,
                Colonia = ?, Calle = ?, CP = ?, Num_Ext = ?, Num_Int = ?
            WHERE Id_Enfermera = ?
        "#,
        &new_vivienda.id_enfermera.to_string(),        
        &new_vivienda.estado.to_string(),
        &new_vivienda.delegacion.to_string(),
        &new_vivienda.colonia.to_string(),
        &new_vivienda.calle.to_string(),
        &new_vivienda.cp.to_string(),        
        &new_vivienda.num_ext.to_string(),        
        &new_vivienda.num_int.to_string(),                
        id_enfermera,                    
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();
    Ok(rows_affected)
}
