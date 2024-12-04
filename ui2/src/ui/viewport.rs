use eframe::egui::{self, Color32, Stroke, Vec2};
use crate::app::ModernEditorApp;

pub fn render_viewport(app: &mut ModernEditorApp, ctx: &egui::Context) {
    egui::CentralPanel::default()
        .frame(egui::Frame {
            fill: app.style.colors["bg_viewport"],
            rounding: app.style.rounding["panel"],
            stroke: app.style.strokes["border_light"],
            ..Default::default()
        })
        .show(ctx, |ui| {
            let viewport_rect = ui.available_rect_before_wrap();
            app.viewport_size = viewport_rect.size();
            
            // Create viewport frame with grid and axis indicators
            ui.painter().rect_filled(
                viewport_rect,
                app.style.rounding["panel"],
                app.style.colors["bg_viewport"],
            );

            // Draw grid
            let grid_size = 50.0;
            let grid_color = Color32::from_rgba_premultiplied(255, 255, 255, 10);
            let grid_strong_color = Color32::from_rgba_premultiplied(255, 255, 255, 25);
            
            for x in (0..(viewport_rect.width() as i32)).step_by(grid_size as usize) {
                let x = x as f32;
                let is_strong_line = x % (grid_size * 5.0) < 1.0;
                ui.painter().line_segment(
                    [
                        viewport_rect.min + Vec2::new(x, 0.0),
                        viewport_rect.min + Vec2::new(x, viewport_rect.height()),
                    ],
                    Stroke::new(
                        if is_strong_line { 1.5 } else { 1.0 },
                        if is_strong_line { grid_strong_color } else { grid_color }
                    ),
                );
            }
            
            for y in (0..(viewport_rect.height() as i32)).step_by(grid_size as usize) {
                let y = y as f32;
                let is_strong_line = y % (grid_size * 5.0) < 1.0;
                ui.painter().line_segment(
                    [
                        viewport_rect.min + Vec2::new(0.0, y),
                        viewport_rect.min + Vec2::new(viewport_rect.width(), y),
                    ],
                    Stroke::new(
                        if is_strong_line { 1.5 } else { 1.0 },
                        if is_strong_line { grid_strong_color } else { grid_color }
                    ),
                );
            }

            // Draw coordinate system
            let axis_length = 60.0;
            let origin = viewport_rect.min + Vec2::new(50.0, viewport_rect.height() - 50.0);
            
            // X axis (red)
            ui.painter().line_segment(
                [origin, origin + Vec2::new(axis_length, 0.0)],
                Stroke::new(2.0, Color32::from_rgb(255, 85, 85)),
            );
            ui.painter().text(
                origin + Vec2::new(axis_length + 5.0, 0.0),
                egui::Align2::LEFT_CENTER,
                "X",
                app.style.fonts["small"].clone(),
                app.style.colors["text_secondary"],
            );
            
            // Y axis (green)
            ui.painter().line_segment(
                [origin, origin + Vec2::new(0.0, -axis_length)],
                Stroke::new(2.0, Color32::from_rgb(85, 255, 85)),
            );
            ui.painter().text(
                origin + Vec2::new(0.0, -(axis_length + 5.0)),
                egui::Align2::CENTER_BOTTOM,
                "Y",
                app.style.fonts["small"].clone(),
                app.style.colors["text_secondary"],
            );

            render_viewport_overlays(app, ui, viewport_rect);
        });
}

fn render_viewport_overlays(app: &ModernEditorApp, ui: &mut egui::Ui, viewport_rect: egui::Rect) {
    // Camera info (top-left)
    let camera_info = format!(
        "Perspective | FOV: 90° | Near: 0.1 | Far: 10000.0"
    );
    let camera_label = egui::Label::new(
        egui::RichText::new(camera_info)
            .font(app.style.fonts["small"].clone())
            .color(app.style.colors["text_secondary"])
    );
    
    let info_rect = egui::Rect::from_min_size(
        viewport_rect.min + Vec2::new(10.0, 10.0),
        Vec2::new(250.0, 20.0),
    );
    ui.put(info_rect, camera_label);

    // Stats overlay (bottom-right)
    let stats_text = format!(
        "FPS: 60 | Draw Calls: 157 | Tris: 24.3K | Lights: 3"
    );
    let stats_label = egui::Label::new(
        egui::RichText::new(stats_text)
            .font(app.style.fonts["small"].clone())
            .color(app.style.colors["text_secondary"])
    );
    
    let stats_rect = egui::Rect::from_min_size(
        viewport_rect.max - Vec2::new(300.0, 30.0),
        Vec2::new(290.0, 20.0),
    );
    ui.put(stats_rect, stats_label);

    // Gizmo overlay (top-right)
    let gizmo_text = format!(
        "Local | Grid Snap: 1.0 | Rotation Snap: 15°"
    );
    let gizmo_label = egui::Label::new(
        egui::RichText::new(gizmo_text)
            .font(app.style.fonts["small"].clone())
            .color(app.style.colors["text_secondary"])
    );
    
    let gizmo_rect = egui::Rect::from_min_size(
        viewport_rect.min + Vec2::new(viewport_rect.width() - 300.0, 10.0),
        Vec2::new(290.0, 20.0),
    );
    ui.put(gizmo_rect, gizmo_label);
}