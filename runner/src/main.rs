use eframe::egui::{self, Vec2, ViewportBuilder};
use std::net::TcpStream;


fn main() {

    let windows = ViewportBuilder{
        title: Some(String::from("RND APP")),
        app_id: Some(String::from("RND APP")),
        // fullsize_content_view: Some(true),
        titlebar_shown: Some(false),
        // min_inner_size: Some(Vec2::new(380., 800.)),
        resizable: Some(true),
        fullscreen:Some(false),
        ..Default::default()
    };
    let options = eframe::NativeOptions {
        viewport:windows,
        // default_theme:Theme::Dark,
        ..Default::default()
    };
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", options , Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
}

#[derive(Default)]
struct MyEguiApp {
    
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    ctx.request_repaint();
       egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Hello World!");
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Add a lot of widgets here.

            });
           
       });
   }
}