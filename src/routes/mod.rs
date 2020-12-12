mod enfermeras;
pub use enfermeras::*;

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(find);
    cfg.service(insert);
    cfg.service(remove);
    cfg.service(update);
}