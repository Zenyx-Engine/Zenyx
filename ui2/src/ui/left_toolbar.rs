use eframe::egui::{self, Rounding};
use crate::app::ModernEditorApp;

pub fn render_left_toolbar(app: &mut ModernEditorApp, ctx: &egui::Context) {
    egui::SidePanel::left("left_toolbar")
        .exact_width(40.0)
        .frame(egui::Frame {
            fill: app.style.colors["toolbar"],
            rounding: Rounding::same(0.0),
            stroke: app.style.strokes["border"],
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(4.0);
                
                for tool in ["Select", "Translate", "Rotate", "Scale"].iter() {
                    let is_active = app.active_tool == *tool;
                    let button = egui::Button::new(
                        egui::RichText::new(*tool)
                            .font(app.style.fonts["small"].clone())
                            .color(if is_active {
                                app.style.colors["accent_primary"]
                            } else {
                                app.style.colors["text_primary"]
                            })
                    )
                    .frame(false)
                    .min_size(egui::vec2(32.0, 32.0));

                    if ui.add(button).clicked() {
                        app.active_tool = tool.to_string();
                    }
                }
            });
        });
}