mod enfermeras;
mod familia;
mod estudios;
mod vivienda;
pub use enfermeras::*;
pub use familia::*;
pub use estudios::*;
pub use vivienda::*;

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    //ENFERMERAS
    cfg.service(obtener_enfermeras);
    cfg.service(insertar_enfermera);
    cfg.service(eliminar_enfermera);
    cfg.service(actualizar_enfermera);
    //FAMILIA
    cfg.service(obtener_familia);
    cfg.service(insertar_familiar);
    cfg.service(eliminar_familiar);
    cfg.service(actualizar_familiar);
    //ESTUDIOS
    cfg.service(obtener_estudios);
    cfg.service(insertar_estudio);
    cfg.service(eliminar_estudio);
    cfg.service(actualizar_estudio);
    //VIVIENDA
    cfg.service(obtener_vivienda);
    cfg.service(insertar_vivienda);
    cfg.service(eliminar_vivienda);
    cfg.service(actualizar_vivienda);
}