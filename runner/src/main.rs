use eframe::egui::{self, menu, Color32, RichText, Vec2, ViewportBuilder};
use egui_tracing::EventCollector;
use tracing::Level;
use tracing::{info,warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::{net::TcpStream, sync::{Arc, Mutex}};
use api::test_thread;


#[derive(Default)]
enum App_menu{
    #[default]
    RAWDATA,
    LOGVIEW,
}
fn main() {
    let collector = egui_tracing::EventCollector::default().with_level(Level::INFO);

    tracing_subscriber::registry()
        .with(collector.clone())
        .init();

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
    eframe::run_native("My egui App", options , Box::new(|cc| 
        {   
            let mut app = MyEguiApp::new(cc,collector);
            test_thread(app.str_mem.clone());
            Ok(Box::new(app))
        }
    ));
}

#[derive(Default)]
struct MyEguiApp {
    app_menu:App_menu,
    collector:EventCollector,
    str_mem:Arc<Mutex<Vec<String>>>,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>,collector:EventCollector) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        // Self::default()
        Self{
            collector,
            app_menu:App_menu::RAWDATA,
            str_mem:Arc::new(Mutex::new(vec![]))
        }
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    ctx.request_repaint();
       egui::CentralPanel::default().show(ctx, |ui| {
        menu::bar(ui, |ui| {
            ui.menu_button(
                RichText::new(
                    "🖥Main"
                    // format!("{} Main", egui_nerdfonts::regular::HOME)
                )
                    .strong()
                    .size(22.0)
                    .color(Color32::from_rgb(38, 150, 255)), 
                |ui|{
                    if ui.button("RAW DATA VIEW").clicked() {
                        self.app_menu = App_menu::RAWDATA;
                    }
                    if ui.button("LOG View").clicked() {
                        self.app_menu = App_menu::LOGVIEW;
                    }
                }
            );
            ui.menu_button(
                RichText::new(
                    "TEMP"
                    // format!("{} Main", egui_nerdfonts::regular::HOME)
                )
                    .strong()
                    .size(22.0)
                    .color(Color32::from_rgb(38, 150, 255)), 
                |ui|{
                    if ui.button("Home").clicked() {
                        // …
                    }
                }
            );
        });
            match self.app_menu {
                App_menu::RAWDATA=>{
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        // Add a lot of widgets here.
                        let list = (*self.str_mem.lock().unwrap()).clone();
                        for i in list{
                            ui.label(i.as_str());
                        }
                        // ui.label(self.str_mem.lock().unwrap().as_str());
                    });
                    egui::SidePanel::right("my_left_panel").show(ctx, |ui| {
                        ui.button("text")
                     });
                },
                App_menu::LOGVIEW=>{
                    egui::ScrollArea::both().show(ui, |ui|{
                        ui.add(egui_tracing::Logs::new(self.collector.clone()))
                    });
                },
            }
       });
   }
}