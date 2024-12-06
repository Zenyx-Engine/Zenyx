use std::sync::Arc;

use anyhow::Result;
use winit::window::Window;

pub struct WgpuCtx<'window> {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'window>,
    surface_config: wgpu::SurfaceConfiguration,
    adapter: wgpu::Adapter,
}

impl<'window> WgpuCtx<'window> {
    pub async fn new(window: Arc<Window>) -> Result<WgpuCtx<'window>> {
        let instnace = wgpu::Instance::default();
        let surface = instnace.create_surface(Arc::clone(&window))?;
        let adapter = instnace
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to obtain render adapter");
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .expect("Failed to create rendering device");

        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);

        let surface_config = surface
            .get_default_config(&adapter, width, height)
            .expect("Failed to get default surface configuration");
        surface.configure(&device, &surface_config);

        Ok(WgpuCtx {
            device,
            queue,
            surface,
            surface_config,
            adapter,
        })
    }

    pub fn new_blocking(window: Arc<Window>) -> Result<WgpuCtx<'window>> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Runtime::new()?.block_on(async { WgpuCtx::new(window).await })
        })
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        let (width, height) = new_size;
        self.surface_config.width = width.max(1);
        self.surface_config.height = height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn draw(&mut self) {
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("Failed to get surface texture");
        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }
        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}
