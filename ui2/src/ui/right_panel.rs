use eframe::egui;
use crate::app::ModernEditorApp;

pub fn render_right_panel(app: &mut ModernEditorApp, ctx: &egui::Context) {
    egui::SidePanel::right("right_panel")
        .resizable(true)
        .default_width(320.0)
        .frame(egui::Frame {
            fill: app.style.colors["bg_secondary"],
            rounding: app.style.rounding["panel"],
            stroke: app.style.strokes["border_light"],
            ..Default::default()
        })
        .show(ctx, |ui| {
            render_outliner(app, ui);
            ui.add_space(app.style.spacings["padding"]);
            render_details(app, ui);
        });
}

fn render_outliner(app: &ModernEditorApp, ui: &mut egui::Ui) {
    egui::CollapsingHeader::new(
        egui::RichText::new("Outliner")
            .font(app.style.fonts["header"].clone())
            .color(app.style.colors["text_primary"])
    )
    .default_open(true)
    .show(ui, |ui| {
        ui.vertical(|ui| {
            for item in ["Main Camera", "DirectionalLight", "Floor", "Player", "Environment"].iter() {
                let label = egui::RichText::new(*item)
                    .font(app.style.fonts["body"].clone())
                    .color(app.style.colors["text_primary"]);
                
                ui.selectable_label(false, label);
            }
        });
    });
}

fn render_details(app: &ModernEditorApp, ui: &mut egui::Ui) {
    egui::CollapsingHeader::new(
        egui::RichText::new("Details")
            .font(app.style.fonts["header"].clone())
            .color(app.style.colors["text_primary"])
    )
    .default_open(true)
    .show(ui, |ui| {
        ui.vertical(|ui| {
            // Transform section
            ui.group(|ui| {
                ui.heading(egui::RichText::new("Transform")
                    .font(app.style.fonts["body"].clone())
                    .color(app.style.colors["text_primary"]));
                
                for (label, value) in [("Location", "0, 0, 0"), ("Rotation", "0, 0, 0"), ("Scale", "1, 1, 1")].iter() {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(*label)
                            .font(app.style.fonts["small"].clone())
                            .color(app.style.colors["text_secondary"]));
                        ui.text_edit_singleline(&mut value.to_string());
                    });
                }
            });

            // Materials section
            ui.group(|ui| {
                ui.heading(egui::RichText::new("Materials")
                    .font(app.style.fonts["body"].clone())
                    .color(app.style.colors["text_primary"]));
                
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Base Color")
                        .font(app.style.fonts["small"].clone())
                        .color(app.style.colors["text_secondary"]));
                });
            });
        });
    });
}