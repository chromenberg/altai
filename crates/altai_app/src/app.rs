use std::{collections::HashMap};

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
}

impl Default for App {
  fn default() -> Self {
    let mut app = App::empty();
    // put the needed app data in here
    app
  }
}

impl App {
  pub fn new() -> App {
    let renderer = altai_render::Renderer::new();
    App::default()
  }

  pub fn empty() -> App {
    Self {
      modules: AppModules {
        main: AppModule::new(),
        sub_modules: HashMap::default()
      },
    }
  }

  // Adds a module
  pub fn add_module() {}
  
  // Adds a window to the main module
  pub fn add_window() {}

  pub fn start(&mut self) {
    
  }

  pub fn suspend(&mut self) {
    
  }

  pub fn resume(&mut self) {
    
  }
  
  pub fn as_ref(&self) -> &Self {
    &self
  }
}

