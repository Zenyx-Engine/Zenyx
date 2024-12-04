mod app;
mod style;
mod ui;

use app::ModernEditorApp;
use eframe::egui::{self, ViewportBuilder};

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1920.0, 1080.0])
            .with_min_inner_size([1280.0, 720.0])
            .with_title("Modern EGUI Editor")
            .with_maximize_button(true)
            .with_decorations(true)
            .with_transparent(false),
        renderer: eframe::Renderer::default(),
        vsync: true,
        multisampling: 4,
        depth_buffer: 24,
        stencil_buffer: 8,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        ..Default::default()
    };

    eframe::run_native(
        "Modern EGUI Editor",
        native_options,
        Box::new(|cc| {
            // Setup custom fonts
            let mut fonts = egui::FontDefinitions::default();
            cc.egui_ctx.set_fonts(fonts);

            // Setup custom style
            let mut style = (*cc.egui_ctx.style()).clone();
            style.spacing.item_spacing = egui::vec2(8.0, 8.0);
            style.spacing.window_margin = egui::Margin::same(12.0);
            style.spacing.button_padding = egui::vec2(12.0, 6.0);
            cc.egui_ctx.set_style(style);

            Ok(Box::<ModernEditorApp>::default())
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    
    let web_options = eframe::WebOptions::default();
    
    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "canvas_id",
            web_options,
            Box::new(|cc| Box::new(ModernEditorApp::default())),
        )
        .await
        .expect("Failed to start eframe");
    });
}