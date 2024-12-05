use eframe::egui::{self, Vec2};
use crate::style::ModernStyle;
use crate::ui::{render_top_toolbar, render_left_toolbar, render_right_panel, 
                render_content_browser, render_viewport};

#[derive(Default)]
pub struct ModernEditorApp {
    pub(crate) style: ModernStyle,
    pub(crate) selected_asset: String,
    pub(crate) viewport_size: Vec2,
    pub(crate) show_content_browser: bool,
    pub(crate) show_outliner: bool,
    pub(crate) show_details: bool,
    pub(crate) is_playing: bool,
    pub(crate) active_tool: String,
}

impl eframe::App for ModernEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut visuals = ctx.style().visuals.clone();
        self.style.clone().apply_to_visuals(&mut visuals);
        ctx.set_visuals(visuals);

        render_top_toolbar(self, ctx);
        render_left_toolbar(self, ctx);
        render_right_panel(self, ctx);
        render_content_browser(self, ctx);
        render_viewport(self, ctx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modern_editor_app_default() {
        let app = ModernEditorApp::default();
        assert_eq!(app.selected_asset, "");
        assert!(!app.is_playing);
        assert!(!app.show_content_browser);
    }
}