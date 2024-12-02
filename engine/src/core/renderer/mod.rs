use log2::{debug, error};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

#[derive(Default)]
pub struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let win_attr = Window::default_attributes().with_title("Zenyx");
            let window = event_loop
                .create_window(win_attr)
                .expect("create window err.");
            self.window = Some(window);
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
            WindowEvent::Resized(size) => {
                let size_str: String = size.height.to_string() + "x" + &size.width.to_string();
                //self.window.as_ref().unwrap().set_title(&format!("you reszed the window to {size_str}"));
                debug!("Window resized to {:?}", size_str);
            }
            _ => error!("Unhandled window event"),
        }
    }
}