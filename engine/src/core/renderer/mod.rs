pub mod ctx;
use ctx::WgpuCtx;

use log::{debug, trace};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

#[derive(Default)]
pub struct App<'window> {
    window: Option<Arc<Window>>,
    ctx: Option<WgpuCtx<'window>>,
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let win_attr = Window::default_attributes().with_title("Zenyx");
            let window = Arc::new(
                event_loop
                    .create_window(win_attr)
                    .expect("create window err."),
            );
            self.window = Some(window.clone());
            let wgpu_ctx = WgpuCtx::new_blocking(window.clone()).unwrap();
            self.ctx = Some(wgpu_ctx)
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
                debug!("Window closed, exiting");
                std::process::exit(0)
            }
            WindowEvent::RedrawRequested => {
                if let Some(ctx) = &mut self.ctx {
                    ctx.draw();
                }
            }
            WindowEvent::Resized(size) => {
                if let (Some(wgpu_ctx), Some(window)) = (&mut self.ctx, &self.window) {
                    wgpu_ctx.resize(size.into());
                    window.request_redraw();

                    let size_str: String = size.height.to_string() + "x" + &size.width.to_string();
                    debug!("Window resized to {:?}", size_str);
                }
            }
            _ => trace!("Unhandled window event"),
        }
    }
}
