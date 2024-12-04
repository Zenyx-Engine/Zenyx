use eframe::egui::{self, Color32, Rounding};
use crate::app::ModernEditorApp;

pub fn render_top_toolbar(app: &mut ModernEditorApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_toolbar")
        .exact_height(32.0)
        .frame(egui::Frame {
            fill: app.style.colors["toolbar"],
            rounding: Rounding::same(0.0),
            stroke: app.style.strokes["border"],
            inner_margin: egui::Margin::symmetric(8.0, 4.0),
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Mode selector
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Editor")
                            .color(app.style.colors["text_primary"])
                            .font(app.style.fonts["body"].clone())
                    );
                    ui.label("▼");
                });

                ui.separator();

                // Menu items
                for menu in ["File", "Edit", "View", "Build", "Window", "Help"].iter() {
                    let button = egui::Button::new(
                        egui::RichText::new(*menu)
                            .font(app.style.fonts["body"].clone())
                            .color(app.style.colors["text_primary"])
                    )
                    .frame(false);
                    
                    if ui.add(button).clicked() {
                        // Menu implementation
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Play controls
                    let play_button = egui::Button::new(
                        egui::RichText::new(if app.is_playing { "■" } else { "▶" })
                            .font(app.style.fonts["body"].clone())
                            .color(Color32::WHITE)
                    )
                    .fill(if app.is_playing {
                        app.style.colors["warning"]
                    } else {
                        app.style.colors["success"]
                    });

                    if ui.add(play_button).clicked() {
                        app.is_playing = !app.is_playing;
                    }
                });
            });
        });
}