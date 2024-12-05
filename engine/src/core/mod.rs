pub mod renderer;
pub mod repl;

use anyhow::Result;
use renderer::App;
use winit::event_loop::{ControlFlow, EventLoop};

pub fn init_renderer() -> Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    Ok(event_loop.run_app(&mut app)?)
}
