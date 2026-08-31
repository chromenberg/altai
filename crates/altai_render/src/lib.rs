pub mod renderer;

use std::sync::Arc;
use vulkano::{
  library::VulkanLibrary,
  instance::Instance,
  instance::InstanceCreateInfo,
};


pub struct Renderer {
  instance: Arc<Instance>,
}

impl Renderer {
  pub fn new() -> Renderer {
    // Load library and create info from altai's cargo
    let library = VulkanLibrary::new().expect("Failed to load Vulkan library.");
    let info = InstanceCreateInfo::application_from_cargo_toml();

    // Create an instance of vulkan
    let instance = Instance::new(library, info).expect("Failed to create a Vulkan instance.");
    
    Self {
      instance: instance
    }
  }

  pub fn instance(&self) -> Arc<Instance> {
    self.instance.clone()
  }
}


fn check_vulkan_support() {
  
}

pub fn init_vulkan() {
   let library = VulkanLibrary::new().unwrap();
   
}

