use eframe::{
    NativeOptions,
    egui::{self, Color32, ColorImage, TextureHandle, Vec2, vec2},
};
use image::{
    ImageBuffer
};

fn main() {
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder {
            fullscreen: Some(false),
            fullsize_content_view: Some(false),
            resizable: Some(false),
            maximize_button: Some(false),
            maximized: Some(false),
            inner_size: Some(vec2(1280.0, 600.0)),
            ..Default::default()
        },
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Droidium", 
        native_options, 
        Box::new(|cc| Ok(Box::new(Droidium::new(cc))))
    );
}

enum Pages {
    Adb,
    Frida,
    LibtoolInject,
}

struct Ui {
    label_font_size: f32,
    heading_font_size: f32,
    watermark_texture: TextureHandle,
    watermark_size: Vec2,
}

struct Droidium {
    current_page: Pages,
    ui: Ui,
}


impl Droidium {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let bytes = include_bytes!("../assets/logo.png");
        let img = image::load_from_memory(bytes).expect("failed to decode");
        let img = img.to_rgba8();
        let (w, h) = img.dimensions();

        let pixels : Vec<Color32> = img.pixels().map(|p| {
            let [r, g, b, a] = p.0;
            Color32::from_rgba_unmultiplied(r, g, b, a)
        }).collect();

        let color_image = ColorImage {
            size: [w as usize, h as usize],
            pixels,
            source_size: Vec2 { x: w as f32, y: h as f32 },
        };

        let tex = cc.egui_ctx.load_texture(
            "background", 
            color_image, 
            egui::TextureOptions::LINEAR,
        );

        let size = tex.size_vec2();

        Self { 
            current_page: Pages::Adb, 
            ui: Ui {
                label_font_size: 14.0,
                heading_font_size: 16.0,

                watermark_texture: tex,
                watermark_size: size,
            } 
        }
    }
}

impl eframe::App for Droidium {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            // wm part
            let rect = ui.max_rect();
            let painter = ui.painter_at(rect);
            
            let img_size = self.ui.watermark_size;
            let center = rect.center();

            let target_rect = egui::Rect::from_center_size(center, img_size);

            painter.image(
                self.ui.watermark_texture.id(), 
                target_rect, 
                egui::Rect::from_min_max(egui::Pos2::new(0.0, 0.0), egui::Pos2::new(1.0, 1.0)), 
                Color32::from_white_alpha(20),
            );
            // end wm part



        });
    }
}