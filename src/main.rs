use altai::app::app::*;
use altai::winit::winit_window;

fn main() {
  let mut app = App::new();
  let mut app_runner = winit_window::WinitAppRunnerState::new(app);
  let mut event_loop = winit::event_loop::EventLoop::builder();
  let built = event_loop.build().expect("failed to build");
  built.run_app(&mut app_runner).expect("failed to start the event loop");
  // let window = windows.create_window(&event_loop, window)
}
