use core::device::vulkan::{VulkanDevice};
use core::device::GraphicsDevice;

pub enum GraphicsBackend {
    Vulkan,
    OpenGL,
    Metal
}

pub struct ApplicationOptions {
    pub graphics_backend: GraphicsBackend,
    pub window_position: (u32, u32),
    pub window_dimensions: (u32, u32),
}

pub struct Application {
    device: Box<GraphicsDevice>
}

impl Application {
    pub fn new(options: &ApplicationOptions) -> Application {
        // 1. Create and initialize any and all physical devices including graphics,
        //    input, sound, etc.
        let device = match options.graphics_backend {
            GraphicsBackend::Vulkan => VulkanDevice::new(),
            _ => unimplemented!()
        };

        // 2. Create and initialize any subsystems required to load the application
        //    including resources managers. These depend on the physical devices
        //    configured in step 1.
        // 3. Create and initialize the simulation world including any component
        //    managers that require subsystems configured in step 2.

        Application {
            device: Box::new(device)
        }
    }
}