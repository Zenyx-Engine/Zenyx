use eframe::egui::{self, vec2};
use crate::app::ModernEditorApp;

pub fn render_content_browser(app: &mut ModernEditorApp, ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("content_browser")
        .resizable(true)
        .default_height(240.0)
        .frame(egui::Frame {
            fill: app.style.colors["bg_secondary"],
            rounding: app.style.rounding["panel"],
            stroke: app.style.strokes["border_light"],
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(app.style.spacings["padding"]);
                ui.heading(
                    egui::RichText::new("Content Browser")
                        .font(app.style.fonts["header"].clone())
                        .color(app.style.colors["text_primary"])
                );
            });

            // Asset path breadcrumb
            ui.horizontal(|ui| {
                ui.add_space(app.style.spacings["padding"]);
                ui.label(
                    egui::RichText::new("/Content/")
                        .font(app.style.fonts["small"].clone())
                        .color(app.style.colors["text_secondary"])
                );
            });

            egui::ScrollArea::horizontal()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        for asset in ["BP_Character", "M_Base", "SK_Mannequin", "T_Texture", "SM_Prop"].iter() {
                            let is_selected = app.selected_asset == *asset;
                            let button = egui::Button::new(
                                egui::RichText::new(*asset)
                                    .font(app.style.fonts["body"].clone())
                                    .color(app.style.colors["text_primary"])
                            )
                            .min_size(vec2(120.0, 120.0))
                            .fill(if is_selected {
                                app.style.colors["selected"]
                            } else {
                                app.style.colors["bg_elevated"]
                            })
                            .rounding(app.style.rounding["button"]);

                            if ui.add(button).clicked() {
                                app.selected_asset = asset.to_string();
                            }
                            
                            ui.add_space(app.style.spacings["padding_small"]);
                        }
                    });
                });
        });
}