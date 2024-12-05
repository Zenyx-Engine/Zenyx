use std::collections::HashMap;
use eframe::egui::{self, Color32, FontId, Rounding, Stroke, Vec2};
use super::Shadow;

#[derive(Clone, Debug)]
pub struct ModernStyle {
    pub colors: HashMap<String, Color32>,
    pub fonts: HashMap<String, FontId>,
    pub spacings: HashMap<String, f32>,
    pub rounding: HashMap<String, Rounding>,
    pub strokes: HashMap<String, Stroke>,
    pub shadows: HashMap<String, Shadow>,
}

impl Default for ModernStyle {
    fn default() -> Self {
        let mut style = ModernStyle {
            colors: HashMap::new(),
            fonts: HashMap::new(),
            spacings: HashMap::new(),
            rounding: HashMap::new(),
            strokes: HashMap::new(),
            shadows: HashMap::new(),
        };

        // Unreal-accurate color palette
        style.colors.insert("bg_primary".to_string(), Color32::from_rgb(40, 40, 40));
        style.colors.insert("bg_secondary".to_string(), Color32::from_rgb(51, 51, 51));
        style.colors.insert("bg_elevated".to_string(), Color32::from_rgb(64, 64, 64));
        style.colors.insert("bg_viewport".to_string(), Color32::from_rgb(20, 20, 20));
        style.colors.insert("accent_primary".to_string(), Color32::from_rgb(0, 122, 204));
        style.colors.insert("accent_secondary".to_string(), Color32::from_rgb(30, 144, 255));
        style.colors.insert("text_primary".to_string(), Color32::from_rgb(220, 220, 220));
        style.colors.insert("text_secondary".to_string(), Color32::from_rgb(128, 128, 128));
        style.colors.insert("border".to_string(), Color32::from_rgb(25, 25, 25));
        style.colors.insert("button_hover".to_string(), Color32::from_rgb(70, 70, 70));
        style.colors.insert("toolbar".to_string(), Color32::from_rgb(51, 51, 51));
        style.colors.insert("selected".to_string(), Color32::from_rgb(60, 60, 60));
        style.colors.insert("success".to_string(), Color32::from_rgb(40, 180, 40));
        style.colors.insert("warning".to_string(), Color32::from_rgb(180, 40, 40));
        style.colors.insert("error".to_string(), Color32::from_rgb(200, 40, 40));

        // Font settings
        style.fonts.insert("header_large".to_string(), FontId::new(20.0, egui::FontFamily::Proportional));
        style.fonts.insert("header".to_string(), FontId::new(16.0, egui::FontFamily::Proportional));
        style.fonts.insert("body".to_string(), FontId::new(13.0, egui::FontFamily::Proportional));
        style.fonts.insert("small".to_string(), FontId::new(11.0, egui::FontFamily::Proportional));
        style.fonts.insert("toolbar".to_string(), FontId::new(12.0, egui::FontFamily::Proportional));

        // Spacings
        style.spacings.insert("padding_large".to_string(), 12.0);
        style.spacings.insert("padding".to_string(), 8.0);
        style.spacings.insert("padding_small".to_string(), 4.0);
        style.spacings.insert("margin".to_string(), 4.0);
        style.spacings.insert("gap".to_string(), 4.0);
        style.spacings.insert("panel_gap".to_string(), 0.0);

        // Rounding
        style.rounding.insert(
            "button".to_string(),
            Rounding {
                nw: 2.0,
                ne: 2.0,
                sw: 2.0,
                se: 2.0,
            },
        );
        style.rounding.insert(
            "panel".to_string(),
            Rounding {
                nw: 0.0,
                ne: 0.0,
                sw: 0.0,
                se: 0.0,
            },
        );

        // Strokes
        style.strokes.insert(
            "border".to_string(),
            Stroke::new(1.0, style.colors["border"]),
        );
        style.strokes.insert(
            "border_light".to_string(),
            Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 8)),
        );

        // Shadows
        style.shadows.insert(
            "dropdown".to_string(),
            Shadow {
                offset: Vec2::new(0.0, 2.0),
                blur: 4.0,
                color: Color32::from_black_alpha(50),
            },
        );
        style.shadows.insert(
            "panel".to_string(),
            Shadow {
                offset: Vec2::new(0.0, 1.0),
                blur: 2.0,
                color: Color32::from_black_alpha(40),
            },
        );

        style
    }
}

impl ModernStyle {
    pub fn apply_to_visuals(self, visuals: &mut egui::Visuals) {
        visuals.window_fill = self.colors["bg_primary"];
        visuals.panel_fill = self.colors["bg_secondary"];
        visuals.faint_bg_color = self.colors["bg_elevated"];
        
        // Widget styling
        visuals.widgets.noninteractive.bg_fill = self.colors["bg_secondary"];
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, self.colors["text_primary"]);
        visuals.widgets.noninteractive.rounding = self.rounding["button"];
        
        visuals.widgets.inactive.bg_fill = self.colors["bg_secondary"];
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, self.colors["text_primary"]);
        visuals.widgets.inactive.rounding = self.rounding["button"];
        
        visuals.widgets.hovered.bg_fill = self.colors["button_hover"];
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, self.colors["text_primary"]);
        visuals.widgets.hovered.rounding = self.rounding["button"];
        
        visuals.widgets.active.bg_fill = self.colors["accent_primary"];
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, self.colors["text_primary"]);
        visuals.widgets.active.rounding = self.rounding["button"];

        // Selection styling
        visuals.selection.bg_fill = self.colors["selected"];
        visuals.selection.stroke = Stroke::new(1.0, self.colors["accent_primary"]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modern_style_default() {
        let style = ModernStyle::default();
        assert!(style.colors.contains_key("bg_primary"));
        assert!(style.fonts.contains_key("header"));
        assert!(style.spacings.contains_key("padding"));
    }
}