use eframe::egui;
use std::sync::Arc;
use winit::window::Window;
use super::ctx::WgpuCtx;  // Updated import path

pub struct ViewportRenderer<'a> {
    wgpu_ctx: Option<WgpuCtx<'a>>,
    texture: Option<egui::TextureHandle>,
    size: (u32, u32),
}

impl<'a> ViewportRenderer<'a> {
    pub fn new() -> Self {
        Self {
            wgpu_ctx: None,
            texture: None,
            size: (800, 600),
        }
    }

    pub fn init(&mut self, window: Arc<Window>) {
        self.wgpu_ctx = Some(WgpuCtx::new_blocking(window).unwrap());
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.size = (width, height);
        if let Some(ctx) = &mut self.wgpu_ctx {
            ctx.resize((width, height));
        }
    }

    pub fn render(&mut self, ctx: &egui::Context) -> Option<&egui::TextureHandle> {
        if let Some(wgpu_ctx) = &mut self.wgpu_ctx {
            // Render the scene
            wgpu_ctx.draw();
            
            // Get the texture data from WGPU
            let pixels = wgpu_ctx.get_texture_data();
            
            // Create or update the egui texture
            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                [self.size.0 as usize, self.size.1 as usize],
                &pixels,
            );

            match &mut self.texture {
                Some(texture) => {
                    texture.set(color_image, egui::TextureOptions::default());
                }
                None => {
                    self.texture = Some(ctx.load_texture(
                        "viewport",
                        color_image,
                        egui::TextureOptions::default()  // Fixed TextureOptions
                    ));
                }
            }
        }
        
        self.texture.as_ref()
    }

    pub fn get_texture(&self) -> Option<&egui::TextureHandle> {
        self.texture.as_ref()
    }
}