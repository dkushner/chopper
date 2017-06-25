pub trait GraphicsDevice {

}

pub mod vulkan {
    use super::GraphicsDevice;

    use std::sync::Arc;
    use vulkano_win;
    use vulkano_win::VkSurfaceBuild;
    use vulkano;
    use winit;
    use vulkano::instance::Instance;
    use vulkano::swapchain;
    use vulkano::device::Device;
    use vulkano::swapchain::{PresentMode, SurfaceTransform, Swapchain};

    #[derive(Debug)]
    pub struct VulkanDevice {
        device: Arc<Device>,
        window: vulkano_win::Window,
    }

    impl VulkanDevice {
        pub fn new() -> VulkanDevice {
            let instance = {
                let extensions = vulkano_win::required_extensions();
                Instance::new(None, &extensions, None).expect("failed to create Vulkan host")
            };

            let physical = vulkano::instance::PhysicalDevice::enumerate(&instance)
                .next().expect("no physical devices found");

            let events = winit::EventsLoop::new();
            let window = winit::WindowBuilder::new().build_vk_surface(&events, instance.clone()).unwrap();

            let queue = physical.queue_families().find(|&q| {
                q.supports_graphics() && window.surface().is_supported(q).unwrap_or(false)
            }).expect("failed to locate supported queue");

            let (device, mut queues) = {
                let extensions = vulkano::device::DeviceExtensions {
                    khr_swapchain: true,
                    ..vulkano::device::DeviceExtensions::none()
                };

                Device::new(&physical, physical.supported_features(), &extensions,
                            [(queue, 0.5)].iter().cloned()).expect("failed to create logical device")
            };

            // TODO: Multi-queue support.
            let queue = queues.next().unwrap();

            let (swapchain, images) = {
                let caps = window.surface().capabilities(physical).expect("");

                // TODO:: Use actual dimensions from options.
                let dimensions = caps.current_extent.unwrap_or([1280, 1024]);
                let alpha = caps.supported_composite_alpha.iter().next().unwrap();
                let format = caps.supported_formats[0].0;

                Swapchain::new(device.clone(), window.surface().clone(), caps.min_image_count,
                               format, dimensions, 1, caps.supported_usage_flags, &queue,
                               SurfaceTransform::Identity, alpha, PresentMode::Fifo,
                               true, None).expect("failed to create swapchain")
            };

            VulkanDevice {
                device: device,
                window: window
            }
        }
    }

    impl GraphicsDevice for VulkanDevice { }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::vulkan::*;

    #[test]
    fn creating_device() {
        let mut device = VulkanDevice::new();
        println!("{:?}", device);
    }
}
