mod updater;
mod app;


fn main() -> eframe::Result<()> {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(eframe::egui::Vec2 { x: 800., y: 640. });
    native_options.centered = false;
    
    eframe::run_native(
        "CDDA更新工具",
        native_options,
        Box::new(|cc| Box::new(app::TemplateApp::new(cc))),
    )
}
