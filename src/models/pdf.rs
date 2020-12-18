use printpdf::*;
use sqlx::MySqlPool;
use std::fs::File;
use image::jpeg::JpegDecoder;
use super::enfermeras;

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
    //INICIOS DE TEXTO   
    current_layer.begin_text_section();    
    current_layer.use_text("Instituto Méxicano del Seguro Social",15, Mm(70.0), Mm(258.0), &font_heveltica_bold);     
    current_layer.use_text("Hospital de Gineco-Obstetricia #4",13, Mm(80.0), Mm(253.0), &font_heveltica_bold);
    //DATOS PERSONALES 
    current_layer.use_text("Datos Personales:", 12,Mm(10.0), Mm(230.0), &font_heveltica_bold);
    let _enfermera = enfermeras::get_enfermera_id(pool, matricula).await.unwrap();
    //PRIMERA LINEA
    current_layer.use_text(format!("Nombre(s): {}", _enfermera.nombres), 11,Mm(10.0), Mm(220.0), &font_heveltica);
    current_layer.use_text(format!("Apellido Materno: {}", _enfermera.apellido_m ), 11,Mm(70.0), Mm(220.0), &font_heveltica); 
    current_layer.use_text(format!("Apellido Paterno: {}", _enfermera.apellido_p), 11,Mm(130.0), Mm(220.0), &font_heveltica);  
    current_layer.use_text(format!("Sexo: {}", _enfermera.sexo), 11,Mm(200.0), Mm(220.0), &font_heveltica);
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
    doc.save(&mut std::io::BufWriter::new(File::create("pdf_pruebas_imss.pdf").unwrap())).unwrap();
    10
}