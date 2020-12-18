use anyhow::Result;
use serde::{ Serialize, Deserialize };
use actix_web::{Error, HttpRequest, HttpResponse, web, Responder};
use futures::future::{ ready, Ready };
use sqlx::{ MySqlPool, FromRow, Row };
use futures::TryStreamExt;
use sqlx::Done;

#[derive(Serialize, FromRow, Deserialize)]
pub struct Enfermera {
    pub matricula: String,
    pub nombres: String,
    pub apellido_m: String,
    pub apellido_p: String,
    pub tipo_enfermera: String,
    pub sexo: String,
    pub jornada: String,
    pub horario_labores: String,
    pub descanso: String,
    pub fecha_nacimiento: String,
    pub lugar_nacimiento: String,
    pub curp: String,
    pub telefono: String,
}
impl Responder for Enfermera {
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
pub async fn get_enfermera_id(pool: &MySqlPool, matricula: &str) -> Result<Enfermera> {
    let mut rows = sqlx::query(
        r#"SELECT * FROM enfermeras 
                WHERE Matricula = ?
            "#)
            .bind(matricula)
            .fetch(pool);
    
    let row = rows.try_next().await.unwrap().unwrap();
    let enfermera = Enfermera {
            matricula: row.get("Matricula"),
            nombres: row.get("Nombres"),
            apellido_m: row.get("Apellido_M"),
            apellido_p: row.get("Apellido_P"),
            tipo_enfermera: row.get("Tipo_Enfermera"),
            sexo:  row.get("Sexo"),
            jornada: row.get("Jornada"),
            horario_labores: row.get("Horario_Labores"),
            descanso: row.get("Descanso"),
            fecha_nacimiento: row.get("Fecha_Nacimiento"),
            lugar_nacimiento: row.get("Lugar_Nacimiento"),
            curp: row.get("Curp"),
            telefono: row.get("Telefono")
        };                           
    Ok(enfermera)
}
pub async fn get_enfermeras(pool: &MySqlPool) -> Result<Vec<Enfermera>> {
    let mut rows = sqlx::query(
        r#"SELECT * FROM enfermeras"#).fetch(pool);
    let mut vec_enfermera: Vec<Enfermera> = vec![];
    while let Some(row) = rows.try_next().await.unwrap() {
        vec_enfermera.push(Enfermera{
            matricula: row.get("Matricula"),
            nombres: row.get("Nombres"),
            apellido_m: row.get("Apellido_M"),
            apellido_p: row.get("Apellido_P"),
            tipo_enfermera: row.get("Tipo_Enfermera"),
            sexo:  row.get("Sexo"),
            jornada: row.get("Jornada"),
            horario_labores: row.get("Horario_Labores"),
            descanso: row.get("Descanso"),
            fecha_nacimiento: row.get("Fecha_Nacimiento"),
            lugar_nacimiento: row.get("Lugar_Nacimiento"),
            curp: row.get("Curp"),
            telefono: row.get("Telefono")
        });                
    };            
    Ok(vec_enfermera)
}
pub async fn set_enfermeras(pool: &MySqlPool, new_enfermera: web::Json<Enfermera>) -> Result<u64,> {
    let rows_affected = sqlx::query!(
        r#"
            INSERT INTO
            enfermeras(Matricula,Nombres,Apellido_M,Apellido_P,
                Tipo_Enfermera,Sexo,Jornada,Horario_Labores,
                Descanso,Fecha_Nacimiento,Lugar_Nacimiento, Curp, Telefono)
            VALUE (?,?,?,?,?,?,?,?,?,?,?,?,?)
        "#,
        &new_enfermera.matricula.to_string(),
        &new_enfermera.nombres.to_string(),
        &new_enfermera.apellido_m.to_string(),
        &new_enfermera.apellido_p.to_string(),
        &new_enfermera.tipo_enfermera.to_string(),
        &new_enfermera.sexo.to_string(),
        &new_enfermera.jornada.to_string(),
        &new_enfermera.horario_labores.to_string(),
        &new_enfermera.descanso.to_string(),
        &new_enfermera.fecha_nacimiento.to_string(),
        &new_enfermera.lugar_nacimiento.to_string(),
        &new_enfermera.curp.to_string(),
        &new_enfermera.telefono.to_string(),
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();    
    Ok(rows_affected)
}
pub async fn delete_enfermera(pool: &MySqlPool, matricula: &str) -> Result<u64,> {    
    let rows_affected = sqlx::query!(
        r#"
            DELETE FROM
                enfermeras
            WHERE Matricula = ?
        "#,
        matricula
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();
    Ok(rows_affected)
}
pub async fn update_enfermera(pool: &MySqlPool, matricula: &str, new_enfermera: web::Json<Enfermera>) -> Result<u64,> {
    let rows_affected = sqlx::query!(
        r#"
            UPDATE enfermeras
                SET Matricula = ?, Nombres = ?, Apellido_M = ?, Apellido_P = ?,
                Tipo_Enfermera = ?, Sexo = ?, Jornada = ?, Horario_Labores = ?,
                Descanso = ?, Fecha_Nacimiento = ?, Lugar_Nacimiento = ?,
                Curp = ?, Telefono = ?
            WHERE Matricula = ?
        "#,
        &new_enfermera.matricula.to_string(),
        &new_enfermera.nombres.to_string(),
        &new_enfermera.apellido_m.to_string(),
        &new_enfermera.apellido_p.to_string(),
        &new_enfermera.tipo_enfermera.to_string(),
        &new_enfermera.sexo.to_string(),
        &new_enfermera.jornada.to_string(),
        &new_enfermera.horario_labores.to_string(),
        &new_enfermera.descanso.to_string(),
        &new_enfermera.fecha_nacimiento.to_string(),
        &new_enfermera.lugar_nacimiento.to_string(),
        &new_enfermera.curp.to_string(),
        &new_enfermera.telefono.to_string(),
        matricula
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();
    Ok(rows_affected)
}

#[derive(Serialize, FromRow, Deserialize)]
pub struct CatalogoTipoEnfermera {
    pub id: String,
    pub descripcion: String
}
impl Responder for CatalogoTipoEnfermera {
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
pub async fn get_catalogo_tipo_enfermera(pool: &MySqlPool) -> Result<Vec<CatalogoTipoEnfermera>> {
    let mut rows = sqlx::query("SELECT * FROM tipo_enfermera").fetch(pool);
    let mut vec_catalogo: Vec<CatalogoTipoEnfermera> = vec![];
    while let Some(row) = rows.try_next().await.unwrap() {
        vec_catalogo.push(CatalogoTipoEnfermera { 
            id: row.get("Id"),
            descripcion: row.get("Descripcion"),       
        });                
    };            
    Ok(vec_catalogo)
}
