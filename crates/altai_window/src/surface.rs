use altai_render::{Renderer, renderer};

use crate::window::Window;
// use glfw::{self, PWindow};

pub struct Surface<'a> {
  /**
   * GLFW_Window
   * Reference to the window provided when creating a window in `window.rs`
   * 
   * Used for Vulkan
   * 
   * ---
   * 
   * TODO: add an appropriate lifetime
   */
  glfw_window: &'a Window
}


impl<'a> Surface<'a> {
  pub fn new(window: &'a Window) -> Surface<'a> {
    // Cannot initialize an empty instance of a surface, as this implies that no `glfw_window` was passed during creation
    // This means that `Surface` cannot actually make the window surface, rendering the entire impl useless
    
    
      let renderer = Renderer::new();
      // window.create_window_surface(renderer.instance(), None, surface)
  
    Surface {
      glfw_window: window // I *would* use the `Window` impl in `window.rs` but that creates a horrendous circular dependency
    }
  }
}