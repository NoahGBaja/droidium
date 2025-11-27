use eframe::{
    NativeOptions,
    egui::{self, Color32, ColorImage, RichText, TextureHandle, Vec2, vec2},
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

    sidepanel_width: f32,
    sidepanel_on: bool,

    padding: f32,
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
                label_font_size: 16.0,
                heading_font_size: 20.0,

                watermark_texture: tex,
                watermark_size: size,

                sidepanel_width: 40.0,
                sidepanel_on: false,

                padding: 10.0,
            } 
        }
    }
}

impl eframe::App for Droidium {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .exact_width(self.ui.sidepanel_width)
            .show(ctx, |ui| {
                ui.add_space(self.ui.padding);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    // ui.add_space(self.ui.padding);
                    let btn = ui.button(RichText::new("☰").color(Color32::WHITE).size(self.ui.heading_font_size));
                    if btn.hovered() { let _ = egui::CursorIcon::PointingHand; }
                    if btn.clicked() { 
                        if self.ui.sidepanel_on { 
                            self.ui.sidepanel_on = false;
                            self.ui.sidepanel_width = 40.0; 
                        }
                        else { 
                            self.ui.sidepanel_on = true;
                            self.ui.sidepanel_width = 160.0; 
                        }
                    }
                });

            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // wm part
            let rect = ui.max_rect();
            let painter = ui.painter_at(rect);
            
            let img_size = self.ui.watermark_size;
            let rectx = (1280.0 / 2.0) - (img_size.x / 2.0);
            let recty = (600.0 / 2.0) - (img_size.y / 2.0);

            let target_rect = egui::Rect::from_min_max(
                egui::pos2(rectx, recty),
                egui::pos2(rectx + self.ui.watermark_size.x, recty + self.ui.watermark_size.y)
            );

            painter.image(
                self.ui.watermark_texture.id(), 
                target_rect,  // this one for the positioning in the panel
                egui::Rect::from_min_max(egui::Pos2::new(0.0, 0.0), egui::Pos2::new(1.0, 1.0)), // this one for the image
                Color32::from_white_alpha(10),
            );
            // end wm part

            ui.horizontal(|ui| {
                ui.set_height(600.0 - self.ui.padding * 2.0);
                ui.group(|ui| {
                    ui.set_width(1280.0 / 2.0 - 40.0);

                    ui.label("kotnol");
                    ui.label("kotnol");
                    ui.label("kotnol");
                });
                ui.group(|ui| {
                    ui.set_width(ui.available_width() - 4.0);
                    
                    ui.label("kotnol");
                    ui.label("kotnol");
                    ui.label("kotnol");
                });
            });

        });
    }
}