use winit::window::WindowLevel;

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