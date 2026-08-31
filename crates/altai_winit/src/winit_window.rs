use std::collections::HashMap;

use winit::{
  application::ApplicationHandler, dpi::{LogicalSize, PhysicalPosition}, event::{DeviceEvent, DeviceId, StartCause, WindowEvent}, event_loop::{ActiveEventLoop, EventLoop}, monitor::{MonitorHandle, VideoModeHandle}, window::{Fullscreen, Window as WinitWindow, WindowId, WindowLevel}
};
use altai_app::app::{self, App};

pub struct Window {
  pub position: WindowPosition,
  pub mode: WindowMode,
  pub window_level: WindowLevel
}

pub struct Vec2 {
  pub x: i64,
  pub y: i64
}

#[derive(Default)]
pub enum WindowPosition {
  #[default]
  Automatic,
  Centered(MonitorSelection),
  At(Vec2)
}

pub enum MonitorSelection {
  Current,
  Primary,
  Index(usize)
}

pub enum VideoMode {

}

pub enum VideoModeSelection {
  Current,
  Specific(VideoMode)
}

#[derive(Default)]
pub enum WindowMode {
  #[default]
  Windowed,
  BorderlessFullscreen(MonitorSelection),
  Fullscreen(MonitorSelection, VideoModeSelection)
}

#[derive(Default)]
pub enum WindowLayer {
  AlwaysOnBottom,

  #[default]
  Normal,
  AlwaysOnTop
}
pub struct WinitWindows {
  pub windows: HashMap<WindowId, WinitWindow>,
}

impl WinitWindows {
  pub fn new() -> Self {
    Self {
      windows: HashMap::new(),
    }
  }

  pub fn create_window(&mut self, event_loop: &ActiveEventLoop, window: Window) -> &mut WinitWindow {
    // get default attr
    let mut window_attributes = WinitWindow::default_attributes();

    // // need to learn this later
    // let selected_monitor = &match window.mode {
    //   WindowMode::BorderlessFullscreen(monitor_selection) | WindowMode::Fullscreen(monitor_selection, _) => 1,
    //   WindowMode::Windowed => None,
    // };
    match window.position {
      WindowPosition::Automatic => {}
      WindowPosition::Centered(_) => {}
      WindowPosition::At(position) => {
        window_attributes = window_attributes.with_position(
          PhysicalPosition::new(position.x as f64, position.y as f64)
        );
      }
    }
    window_attributes = window_attributes.with_inner_size(LogicalSize::new(640, 480));

    window_attributes = window_attributes
      .with_window_level(window.window_level)
      .with_transparent(true)
      .with_blur(true)
      .with_resizable(true);

    let winit_window = event_loop.create_window(window_attributes).expect("Failed to create window");
    winit_window.set_visible(true);


    self.windows.entry(winit_window.id()).insert_entry(winit_window).into_mut()
  }
}

pub enum AppLifecycle {
  Idle,
  Running,
  WillSuspend,
  Suspended,
  WillResume
}

pub struct WinitAppRunnerState {
  app: altai_app::app::App,
  lifecycle: AppLifecycle,
  previous_lifecycle: AppLifecycle,
  windows: WinitWindows
}

impl  WinitAppRunnerState {
  pub fn new(app: App) -> Self {
    Self {
      app: app,
      lifecycle: AppLifecycle::Idle,
      previous_lifecycle: AppLifecycle::Idle,
      windows: WinitWindows::new()
    }
  }

  pub fn windows_ref(&self) -> &WinitWindows {
    &self.windows
  }

  
  pub fn windows_mut(&mut self) -> &mut WinitWindows {
    &mut self.windows
  }
}


pub enum WinitUserEvent {
    /// Dummy event that just wakes up the winit event loop
    WakeUp,
    /// Tell winit that a window needs to be created
    WindowAdded,
}

impl ApplicationHandler for WinitAppRunnerState {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    self.windows_mut().create_window(
      event_loop,
      Window {
        position: {WindowPosition::At(
          Vec2 { x: 5, y: 5}
        )},
        mode: WindowMode::Windowed,
        window_level: WindowLevel::AlwaysOnTop
      });
    
    self.app.resume();
  }

  fn suspended(&mut self, _: &ActiveEventLoop) {
    self.app.suspend();
  }

  fn user_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    event: ()
  ) {}

  fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    window_id: WindowId,
    event: WindowEvent
  ) {}
  
  fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
    let _ = event_loop;
  }
  
  fn exiting(&mut self, event_loop: &ActiveEventLoop) {
    let _ = event_loop;
  }
  
  fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
    let _ = event_loop;
  }
  
  fn device_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    device_id: DeviceId,
    event: DeviceEvent,
  ) {
    let _ = (event_loop, device_id, event);
  }
  
  fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
    let _ = (event_loop, cause);
  }
}
