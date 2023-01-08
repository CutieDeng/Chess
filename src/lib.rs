use eframe::egui;

pub fn setup_fonts(ctx : &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();  
    fonts.font_data.insert("font1".into(), egui::FontData::from_static(
        include_bytes!("/Users/cutiedeng/downloads/lxgw/LXGWWenKai-Regular.ttf")
    )); 
    fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "font1".to_owned()); 
    fonts.families.entry(egui::FontFamily::Monospace).or_default().push("font1".to_owned()); 
    ctx.set_fonts(fonts); 
}