use std::ops::Deref;

use glfw::{self, Context, Glfw, PWindow};

const DEFAULT_WINDOW_SIZE: Size = Size{ width: 640, height: 480 };

pub struct Size {
  pub width: u32,
  pub height: u32
}

pub struct Window {
  pub size: Size,
  pub glfw_window: Option<PWindow>
}

 impl Default for Window {
  fn default() -> Self {
    let mut window = Window::empty();
    window.size = DEFAULT_WINDOW_SIZE;
    
    window
  }
}

 impl Window {
  pub fn empty() -> Window {
    Self {
      size: Size { width: 0, height: 0 },
      glfw_window: None
    }
  }

  pub fn create(glfw_obj: &mut Glfw, width: u32, height: u32) -> Window {
    let title = "Altai Window";
    
    let mut glfw_window = glfw::Glfw::create_window(glfw_obj, width, height, title, glfw::WindowMode::Windowed).unwrap().0;
    let mut window = Window::empty();
    window.glfw_window = Some(glfw_window);

    // TODO: Don't use opengl
    window
  }

  pub fn should_close(&self) -> bool {
    self.glfw_window
      .as_ref()
      .map_or(false, |window| window.should_close())
  }


  // returns a reference to the stored PWindow
  pub fn glfw_window(&self) -> &PWindow {
    self.glfw_window.as_ref().unwrap()
  }

  // returns a mutable reference to the stored PWindow
  pub fn glfw_window_mut(&mut self) -> &mut PWindow {
    self.glfw_window.as_mut().unwrap()
  }
}
