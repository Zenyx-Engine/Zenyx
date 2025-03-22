use std::sync::Arc;

use ctx::WgpuCtx;
use log::{debug, trace};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};
pub mod ctx;

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
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::Resized(size) => {
                if let (Some(wgpu_ctx), Some(window)) = (&mut self.ctx, &self.window) {
                    wgpu_ctx.resize(size.into());
                    window.request_redraw();
                    let size_str: String = size.height.to_string() + "x" + &size.width.to_string();
                    //self.window.as_ref().unwrap().set_title(&format!("you reszed the window to
                    // {size_str}"));
                    debug!("Window resized to {:?}", size_str);
                }
            }
            _ => trace!("Unhandled window event"),
        }
    }
}

pub fn init_renderer(event_loop: EventLoop<()>) {
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
