use std::{collections::HashMap, mem};
use glfw::{Context, Glfw, InitError};
use altai_window::window::Window;

#[derive(Default)]
struct AppModule {

}

impl AppModule {
  pub fn new() -> AppModule {
    AppModule::default()
  }
}

// This contains every registered module for an Altai App
struct AppModules {
  // This is the main module for the engine, this is always present
  main: AppModule,

  // All other modules are registered under sub_modules
  // Temporarily indexed by string
  sub_modules: HashMap<String, AppModule>
}

pub struct App {
  modules: AppModules,
  temp_window: Window
}

impl Default for App {
  fn default() -> Self {
    let mut app = App::empty();
    // put the needed app data in here
    let mut glfw_object = glfw::init_no_callbacks().unwrap();
    
    app.temp_window = Window::create(&mut glfw_object, 640, 480);
    
    // return the app
    app
  }
}

impl App {
  pub fn new() -> App {
    App::default()
  }

  pub fn empty() -> App {
    Self {
      modules: AppModules {
        main: AppModule::new(),
        sub_modules: HashMap::default()
      },
      temp_window: Window::empty()
    }
  }

  // Adds a module
  pub fn add_module() {}
  
  // Adds a window to the main module
  pub fn add_window() {}

  pub fn start(&mut self) {
    self.temp_window.glfw_window_mut().make_current();
    while !self.temp_window.glfw_window().should_close() {
      
    }
  }
}
