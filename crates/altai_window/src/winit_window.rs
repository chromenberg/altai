// use std::collections::HashMap;

// use winit::{
//   dpi::{LogicalSize, PhysicalPosition},
//   event_loop::{ActiveEventLoop, EventLoop},
//   monitor::{MonitorHandle, VideoModeHandle},
//   event::{WindowEvent},
//   window::{Fullscreen, Window as WinitWindow, WindowId, WindowLevel},
//   application::ApplicationHandler
// };

// use altai_app::app::{self, App, WinitUserEvent};
// use crate::{window::{Window, WindowMode, WindowPosition}};


// pub struct WinitWindows {
//   pub windows: HashMap<WindowId, WinitWindow>,
// }

// impl WinitWindows {
//   pub fn new() -> Self {
//     Self {
//       windows: HashMap::new(),
//     }
//   }

//   pub fn create_window(&mut self, event_loop: &ActiveEventLoop, window: Window) -> &mut WinitWindow {
//     // get default attr
//     let mut window_attributes = WinitWindow::default_attributes();

//     // // need to learn this later
//     // let selected_monitor = &match window.mode {
//     //   WindowMode::BorderlessFullscreen(monitor_selection) | WindowMode::Fullscreen(monitor_selection, _) => 1,
//     //   WindowMode::Windowed => None,
//     // };
//     match window.position {
//       WindowPosition::Automatic => {}
//       WindowPosition::Centered(_) => {}
//       WindowPosition::At(position) => {
//         window_attributes = window_attributes.with_position(
//           PhysicalPosition::new(position.x as f64, position.y as f64)
//         );
//       }
//     }
//     window_attributes = window_attributes.with_inner_size(LogicalSize::new(640, 480));

//     window_attributes = window_attributes
//       .with_window_level(window.window_level)
//       .with_decorations(false)
//       .with_transparent(true)
//       .with_blur(true)
//       .with_resizable(true);
    
//     let winit_window = event_loop.create_window(window_attributes).expect("Failed to create window");
//     winit_window.set_visible(true);

    
//     self.windows.entry(winit_window.id()).insert_entry(winit_window).into_mut()
//   }
// }

// pub enum AppLifecycle {
//   Idle,
//   Running,
//   WillSuspend,
//   Suspended,
//   WillResume
// }

// pub struct WinitAppRunnerState {
//   app: altai_app::app::App,
//   lifecycle: AppLifecycle,
//   previous_lifecycle: AppLifecycle,
  
// }

// impl WinitAppRunnerState {
//   fn new(mut app: App) -> Self {
//     Self {
//       app,
//       lifecycle: AppLifecycle::Idle,
//       previous_lifecycle: AppLifecycle::Idle,
//     }
//   }
// }
// impl ApplicationHandler<WinitUserEvent> for WinitAppRunnerState {
//   fn resumed(&mut self, _: &ActiveEventLoop) {
//     self.app.resume();
//   }

//   fn suspended(&mut self, _: &ActiveEventLoop) {
//     self.app.suspend();
//   }

//   fn user_event(&mut self, _: &ActiveEventLoop, event: WinitUserEvent) {
    
//   }

//   fn window_event(&mut self, _: &ActiveEventLoop, event: WindowEvent) {
//     println!(event)
//   }
// }