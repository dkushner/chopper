use gfx::{self, Device, Factory};
use winit::{self, Window};
use core::platform::{Platform, PlatformIdentity};
use glutin;

use gfx_window_glutin;

#[cfg(feature = "metal")]
use gfx_window_metal;

pub struct ApplicationOptions {
    pub platform: PlatformIdentity,
    pub window_position: (u32, u32),
    pub window_dimensions: (u32, u32),
    pub title: String,
}

pub struct Application<D: Device, F: Factory<D::Resources>> {
    window: Window,
    platform: Platform<D, F>,
}

impl <D: Device, F: Factory<D::Resources>> Application<D, F> {
    pub fn new(options: &ApplicationOptions) -> Self {
        let winit_builder = winit::WindowBuilder::new()
            .with_dimensions(options.window_dimensions.0, options.window_dimensions.1)
            .with_title(options.title.as_str());


        let (window, mut device, mut factory, color, depth) = match options.platform {
            PlatformIdentity::OpenGL => {
                let events = glutin::EventsLoop::new();
                let gl_version = glutin::GlRequest::GlThenGles {
                    opengl_version: (3, 2),
                    opengles_version: (2, 0),
                };

                let builder = glutin::WindowBuilder::from_winit_builder(winit_builder)
                    .with_gl(gl_version)
                    .with_vsync();

                gfx_window_glutin::init::<gfx::format::Rgba8, gfx::format::DepthStencil>(builder, &events)
            },
            PlatformIdentity::Metal => {
                use gfx::texture::Size;

                let (window, mut device, mut factory, color) = gfx_window_metal::init::<gfx::format::Rgba8>(winit_builder);
                let (width, height) = window.get_inner_size_points().unwrap();
                let depth = factory.create_depth_stencil_view_only(width as Size, height as Size).unwrap;

                (window, device, factory, color, depth)
            }
            _ => panic!()
        };


        let encoder = factory.create_command_buffer().into();

        Application {
            window: window,
            platform: Platform::new(device, factory, encoder),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
