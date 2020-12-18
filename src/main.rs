use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{App, HttpServer};
use sqlx::MySqlPool;
mod routes;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db_pool = MySqlPool::connect("mysql://root:VacaLoca@127.0.0.1:3306/enfermeria")
        .await
        .unwrap();
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    // .allowed_origin("http://localhost:8084")
                    // .allowed_origin("http://localhost:8083")
                    .allow_any_origin()                    
                    .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)                                             
                    .max_age(3600),
            )
            .data(db_pool.clone())
            .configure(routes::init)                   
    })
    .bind("localhost:4000")?
    .run()
    .await
}
