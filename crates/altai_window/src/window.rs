use glfw::{self, Context, Glfw, GlfwReceiver, PWindow, WindowEvent};

const DEFAULT_WINDOW_SIZE: Size = Size{ width: 640, height: 480 };

pub struct Size {
  pub width: u32,
  pub height: u32
}

pub struct Window {
  pub size: Size,
  // pub glfw_window: PWindow
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
    }
  }

  pub fn create(glfw_obj: &mut Glfw, width: u32, height: u32) -> Window {
    let title = "Altai Window";
    
    let mut glfw_window = glfw::Glfw::create_window(glfw_obj, width, height, title, glfw::WindowMode::Windowed).unwrap().0;
    let mut window = Window::empty();
    // window.glfw_window = glfw_window;

    glfw_window.make_current();
    
    while !&glfw_window.should_close() {
      
    }
    window
  }


}
