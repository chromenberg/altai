use std::collections::HashMap;

use glfw;

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

struct App {
  modules: AppModules,
}

impl Default for App {
  fn default() -> Self {
    let app = App::empty();
    // put the needed app data in here

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
      }
    }
  }
}
