use actix_web::Result;
use printpdf::*;
use sqlx::MySqlPool;
use std::fs::File;
use image::jpeg::JpegDecoder;
use super::enfermeras;
use super::vivienda;
use super::estudios;
use super::familia;

pub async fn imprimir_pdf_enfermera(pool: &MySqlPool, matricula: &str) -> i32 {
    let (doc, page1, layer1) = PdfDocument::new("printpdf graphics test", Mm(228.6), Mm(279.4), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    //LOGO DEL IMSS    
    let mut image_file = File::open("./static/logo-imss.jpg").unwrap();  
    let image = Image::try_from(JpegDecoder::new(&mut image_file).unwrap()).unwrap();            
    image.add_to_layer(current_layer.clone(),Some(Mm(10.0)), Some(Mm(250.0)), None, None, None, Some(600.0));              
    //TIPOS DE LETRAS
    let font_heveltica_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap(); 
    let font_heveltica = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();     
    //TEXTO CABECERA  
    current_layer.begin_text_section();    
    current_layer.use_text("Instituto Méxicano del Seguro Social",15, Mm(70.0), Mm(258.0), &font_heveltica_bold);     
    current_layer.use_text("Hospital de Gineco-Obstetricia #4",13, Mm(80.0), Mm(253.0), &font_heveltica_bold);    
    current_layer.end_text_section();     
    //DATOS GENERALES
    let enfermera = enfermeras::get_enfermera_id(pool, matricula).await.unwrap();
    datos_generales_pdf(&enfermera, &current_layer, &font_heveltica_bold, &font_heveltica).unwrap();
    //DATOS VIVIENDA
    let vivienda = vivienda::get_vivienda(pool, matricula).await.unwrap();
    datos_vivienda_pdf(&vivienda, &current_layer, &font_heveltica_bold, &font_heveltica).unwrap();    
    //DATOS ACADEMICOS
    let estudios = estudios::get_estudios(pool, matricula).await.unwrap();
    let eje_y = datos_academicos_pdf(estudios, &current_layer, &font_heveltica_bold, &font_heveltica).unwrap();
    //DATOS FAMILIA
    let familia = familia::get_familia(pool, matricula).await.unwrap();
    datos_familla_pdf(familia, &current_layer, &font_heveltica_bold, &font_heveltica, eje_y).unwrap();
    //GUARDAR PDF
    doc.save(&mut std::io::BufWriter::new(File::create("pdf_pruebas_imss.pdf").unwrap())).unwrap();    
    10
}
fn datos_generales_pdf(_enfermera: &enfermeras::Enfermera, current_layer: &PdfLayerReference, font_heveltica_bold: &IndirectFontRef, font_heveltica: &IndirectFontRef) -> Result<()>{    
    current_layer.begin_text_section();  
    current_layer.use_text("Datos Personales:", 12,Mm(10.0), Mm(230.0), &font_heveltica_bold);    
    //PRIMERA LINEA    
    current_layer.use_text(format!("Nombre(s): {}", _enfermera.nombres), 11,Mm(10.0), Mm(220.0), &font_heveltica);
    current_layer.use_text(format!("Apellido Materno: {}", _enfermera.apellido_m ), 11,Mm(70.0), Mm(220.0), &font_heveltica); 
    current_layer.use_text(format!("Apellido Paterno: {}", _enfermera.apellido_p), 11,Mm(130.0), Mm(220.0), &font_heveltica);  
    current_layer.use_text(format!("Sexo: {}", _enfermera.sexo), 11,Mm(180.0), Mm(220.0), &font_heveltica);
    //SEGUNDA LINEA
    current_layer.use_text(format!("Lugar de Nacimiento: {}", _enfermera.lugar_nacimiento), 11,Mm(10.0), Mm(210.0), &font_heveltica);
    current_layer.use_text(format!("Fecha de Nacimiento: {}", _enfermera.fecha_nacimiento), 11,Mm(75.0), Mm(210.0), &font_heveltica);
    current_layer.use_text(format!("Curp: {}", _enfermera.curp), 11,Mm(150.0), Mm(210.0), &font_heveltica);
    //TERCERA LINEA
    current_layer.use_text(format!("Tipo de Enfermera: {}", _enfermera.tipo_enfermera), 11,Mm(10.0), Mm(200.0), &font_heveltica);
    current_layer.use_text(format!("Jornada: {}", _enfermera.jornada), 11,Mm(65.0), Mm(200.0), &font_heveltica);
    current_layer.use_text(format!("Horario Laboral: {}", _enfermera.horario_labores), 11,Mm(100.0), Mm(200.0), &font_heveltica);
    current_layer.use_text(format!("Descanso: {}", _enfermera.descanso), 11,Mm(160.0), Mm(200.0), &font_heveltica);
    //CUARTA LINEA
    current_layer.use_text(format!("Telefono: {}", _enfermera.telefono), 11,Mm(10.0), Mm(190.0), &font_heveltica);
    current_layer.end_text_section(); 
    //FINAL DEL TEXTO
    Ok(())
}
fn datos_vivienda_pdf(_vivienda: &vivienda::Vivienda, current_layer: &PdfLayerReference, font_heveltica_bold: &IndirectFontRef, font_heveltica: &IndirectFontRef) -> Result<()>{    
    current_layer.begin_text_section();  
    current_layer.use_text("Datos Vivienda:", 12,Mm(10.0), Mm(180.0), &font_heveltica_bold);    
    //PRIMERA LINEA
    current_layer.use_text(format!("Estado: {}", _vivienda.estado),11,Mm(10.0), Mm(170.0), &font_heveltica);
    current_layer.use_text(format!("Delegacion: {}", _vivienda.delegacion),11,Mm(60.0), Mm(170.0), &font_heveltica);        
    current_layer.use_text(format!("Colonia: {}", _vivienda.colonia),11,Mm(110.0), Mm(170.0), &font_heveltica);    
    //SEGUNDA LINEA
    current_layer.use_text(format!("Calle(s): {}", _vivienda.calle),11,Mm(10.0), Mm(160.0), &font_heveltica);
    current_layer.use_text(format!("C.P: {}", _vivienda.cp),11,Mm(60.0), Mm(160.0), &font_heveltica);
    current_layer.use_text(format!("Numero Ext: {}", _vivienda.num_ext),11,Mm(100.0), Mm(160.0), &font_heveltica);
    current_layer.use_text(format!("Numero Int: {}", _vivienda.num_int),11,Mm(140.0), Mm(160.0), &font_heveltica);
    current_layer.end_text_section();
    Ok(())
}
fn datos_academicos_pdf(_estudios: Vec<estudios::Estudio>, current_layer: &PdfLayerReference, font_heveltica_bold: &IndirectFontRef, font_heveltica: &IndirectFontRef) -> Result<f64> {
    current_layer.begin_text_section();
    current_layer.use_text("Datos Academicos:", 12,Mm(10.0), Mm(150.0), &font_heveltica_bold);
    //PRIMERA LINEA
    current_layer.use_text("Nivel",11,Mm(10.0), Mm(140.0), &font_heveltica);
    current_layer.use_text("Titulo",11,Mm(40.0), Mm(140.0), &font_heveltica);    
    current_layer.use_text("Institucion",11,Mm(80.0), Mm(140.0), &font_heveltica);
    current_layer.use_text("Tipo de Escuela",11,Mm(110.0), Mm(140.0), &font_heveltica);
    let mut espacio = 130_f64;
    for estudio in _estudios {
        current_layer.use_text(estudio.nivel ,11,Mm(10.0), Mm(espacio), &font_heveltica);
        current_layer.use_text(estudio.titulo,11,Mm(40.0), Mm(espacio), &font_heveltica);    
        current_layer.use_text(estudio.institucion,11,Mm(80.0), Mm(espacio), &font_heveltica);
        current_layer.use_text(estudio.tipo_escuela,11,Mm(110.0), Mm(espacio), &font_heveltica);
        espacio -= 10_f64;
    }
    current_layer.end_text_section();
    Ok(espacio)
}
fn datos_familla_pdf(_familia: Vec<familia::Familia>, current_layer: &PdfLayerReference, font_heveltica_bold: &IndirectFontRef, font_heveltica: &IndirectFontRef, espacio: f64) -> Result<()> {
    current_layer.begin_text_section();
    current_layer.use_text("Datos Academicos:", 12,Mm(10.0), Mm(espacio), &font_heveltica_bold);
    //PRIMERA LINEA
    current_layer.use_text("Nombre Completo",11,Mm(10.0), Mm(espacio - 10_f64), &font_heveltica);
    current_layer.use_text("Parentesco",11,Mm(80.0), Mm(espacio - 10_f64), &font_heveltica);            
    let mut _espacio = espacio - 20_f64;
    for familiar in _familia {
        current_layer.use_text(format!("{} {} {}", familiar.nombres, familiar.apellido_m, familiar.apellido_p) ,11,Mm(10.0), Mm(_espacio), &font_heveltica);
        current_layer.use_text(familiar.parentesco,11,Mm(80.0), Mm(_espacio), &font_heveltica);                    
        _espacio -= 10_f64;
    }
    current_layer.end_text_section();
    Ok(())
}