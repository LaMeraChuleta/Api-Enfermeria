mod enfermeras;
mod familia;
pub use enfermeras::*;
pub use familia::*;

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(obtener_enfermeras);
    cfg.service(insertar_enfermera);
    cfg.service(eliminar_enfermera);
    cfg.service(actualizar_enfermera);
    cfg.service(obtener_familia);
    cfg.service(insertar_familiar);
    cfg.service(eliminar_familiar);
    cfg.service(actualizar_familiar);
}