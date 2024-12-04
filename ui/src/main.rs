use eframe::egui::{self, Color32, FontId, Rounding, Stroke, Vec2, viewport, ViewportBuilder};
use std::collections::HashMap;

// Define our custom style system
#[derive(Clone, Debug)]
pub struct PulsarStyle {
    colors: HashMap<String, Color32>,
    fonts: HashMap<String, FontId>,
    spacings: HashMap<String, f32>,
    rounding: HashMap<String, Rounding>,
    strokes: HashMap<String, Stroke>,
}

impl Default for PulsarStyle {
    fn default() -> Self {
        let mut style = PulsarStyle {
            colors: HashMap::new(),
            fonts: HashMap::new(),
            spacings: HashMap::new(),
            rounding: HashMap::new(),
            strokes: HashMap::new(),
        };

        // Define Unreal-like colors
        style.colors.insert("bg_dark".to_string(), Color32::from_rgb(25, 25, 25));
        style.colors.insert("bg_light".to_string(), Color32::from_rgb(35, 35, 35));
        style.colors.insert("bg_viewport".to_string(), Color32::from_rgb(20, 20, 20));
        style.colors.insert("accent".to_string(), Color32::from_rgb(0, 120, 215));
        style.colors.insert("text_primary".to_string(), Color32::from_rgb(200, 200, 200));
        style.colors.insert("text_secondary".to_string(), Color32::from_rgb(150, 150, 150));
        style.colors.insert("border".to_string(), Color32::from_rgb(45, 45, 45));
        style.colors.insert("button_hover".to_string(), Color32::from_rgb(60, 60, 60));
        style.colors.insert("toolbar".to_string(), Color32::from_rgb(40, 40, 40));
        style.colors.insert("selected".to_string(), Color32::from_rgb(70, 70, 70));

        // Define common fonts
        style.fonts.insert("header".to_string(), FontId::new(20.0, egui::FontFamily::Proportional));
        style.fonts.insert("body".to_string(), FontId::new(14.0, egui::FontFamily::Proportional));
        style.fonts.insert("small".to_string(), FontId::new(12.0, egui::FontFamily::Proportional));
        style.fonts.insert("toolbar".to_string(), FontId::new(13.0, egui::FontFamily::Proportional));

        // Define common spacings
        style.spacings.insert("padding".to_string(), 8.0);
        style.spacings.insert("margin".to_string(), 4.0);
        style.spacings.insert("gap".to_string(), 6.0);
        style.spacings.insert("panel_gap".to_string(), 1.0);

        // Define common roundings
        style.rounding.insert(
            "button".to_string(),
            Rounding {
                nw: 2.0,
                ne: 2.0,
                sw: 2.0,
                se: 2.0,
            },
        );

        // Define common strokes
        style.strokes.insert(
            "border".to_string(),
            Stroke::new(1.0, style.colors["border"]),
        );

        style
    }
}

impl PulsarStyle {
    pub fn apply_to_visuals(self, visuals: &mut egui::Visuals) {
        visuals.window_fill = self.colors["bg_dark"];
        visuals.panel_fill = self.colors["bg_light"];
        visuals.faint_bg_color = self.colors["bg_light"];
        visuals.widgets.noninteractive.bg_fill = self.colors["bg_light"];
        visuals.widgets.inactive.bg_fill = self.colors["bg_light"];
        visuals.widgets.hovered.bg_fill = self.colors["button_hover"];
        visuals.widgets.active.bg_fill = self.colors["accent"];
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, self.colors["text_primary"]);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, self.colors["text_primary"]);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, self.colors["text_primary"]);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, self.colors["text_primary"]);
    }
}

// Unreal Editor-like app
#[derive(Default)]
pub struct UnrealEditorApp {
    style: PulsarStyle,
    selected_asset: String,
    viewport_size: Vec2,
    show_content_browser: bool,
    show_outliner: bool,
    show_details: bool,
}

impl eframe::App for UnrealEditorApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Apply the custom style
        let mut visuals = ctx.style().visuals.clone();
        self.style.clone().apply_to_visuals(&mut visuals);
        ctx.set_visuals(visuals);

        // Top toolbar
        egui::TopBottomPanel::top("top_toolbar")
            .exact_height(32.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("File").clicked() {
                        // File menu implementation
                    }
                    if ui.button("Edit").clicked() {
                        // Edit menu implementation
                    }
                    ui.separator();
                    
                    let play_button = egui::Button::new("▶ Play")
                        .fill(self.style.colors["accent"])
                        .rounding(self.style.rounding["button"]);
                    if ui.add(play_button).clicked() {
                        // Play implementation
                    }
                });
            });

        // Left toolbar
        egui::SidePanel::left("left_toolbar")
            .exact_width(40.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    for tool in ["Select", "Translate", "Rotate", "Scale"].iter() {
                        if ui.button(*tool).clicked() {
                            // Tool implementation
                        }
                    }
                });
            });

        // Right panel (Outliner & Details)
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(300.0)
            .show(ctx, |ui| {
                // Outliner
                egui::CollapsingHeader::new("Outliner")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.label("World Outliner");
                        // Add outliner items here
                    });

                ui.separator();

                // Details panel
                egui::CollapsingHeader::new("Details")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.label("Details");
                        // Add property editors here
                    });
            });

        // Content browser at bottom
        egui::TopBottomPanel::bottom("content_browser")
            .resizable(true)
            .default_height(200.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Content Browser");
                });
                
                // Content grid
                egui::ScrollArea::horizontal().show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        for asset in ["BP_Character", "M_Base", "SK_Mannequin"].iter() {
                            let is_selected = self.selected_asset == *asset;
                            let mut button = egui::Button::new(*asset)
                                .fill(if is_selected {
                                    self.style.colors["selected"]
                                } else {
                                    self.style.colors["bg_light"]
                                });
                            
                            if ui.add(button).clicked() {
                                self.selected_asset = asset.to_string();
                            }
                        }
                    });
                });
            });

        // Main viewport
        egui::CentralPanel::default().show(ctx, |ui| {
            let viewport_rect = ui.available_rect_before_wrap();
            self.viewport_size = viewport_rect.size();
            
            // Viewport frame
            ui.painter().rect_filled(
                viewport_rect,
                0.0,
                self.style.colors["bg_viewport"],
            );
            
            // Viewport overlay text
            ui.put(
                viewport_rect,
                egui::Label::new(
                    egui::RichText::new("Perspective")
                        .color(self.style.colors["text_secondary"])
                        .font(self.style.fonts["small"].clone())
                ),
            );
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Unreal Editor Style",
        native_options,
        Box::new(|cc| Ok(Box::<UnrealEditorApp>::default()))
    )
}