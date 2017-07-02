use gfx::{self, Device, Factory};
use winit::{self, Window};
use core::platform::{Platform, PlatformIdentity};
use glutin;

use gfx_device_gl;
use gfx_window_glutin;

#[cfg(feature = "metal")]
use gfx_window_metal;

#[cfg(feature = "metal")]
use gfx_device_metal;


pub struct ApplicationOptions {
    pub platform: PlatformIdentity,
    pub window_position: (u32, u32),
    pub window_dimensions: (u32, u32),
    pub title: String,
}

pub struct Application;

impl Application {
    pub fn new(options: &ApplicationOptions) -> Box<ApplicationProxy> {
        create_application(options)
    }
}

pub trait ApplicationProxy {
    fn start(&self);
}

pub struct ApplicationBase<D: Device, F: Factory<D::Resources>> {
    platform: Platform<D, F>,
}

impl <D, F> ApplicationProxy for ApplicationBase<D, F> where D: Device, F: Factory<D::Resources> {
    fn start(&self) { }
}

fn create_application(options: &ApplicationOptions) -> Box<ApplicationProxy> {
    let winit_builder = winit::WindowBuilder::new()
        .with_dimensions(options.window_dimensions.0, options.window_dimensions.1)
        .with_title(options.title.as_str());


    match options.platform {
        PlatformIdentity::OpenGL => {
            let events = glutin::EventsLoop::new();
            let gl_version = glutin::GlRequest::GlThenGles {
                opengl_version: (3, 2),
                opengles_version: (2, 0),
            };

            let builder = glutin::WindowBuilder::from_winit_builder(winit_builder)
                .with_gl(gl_version)
                .with_vsync();

            let (window, mut device, mut factory, color, depth) = gfx_window_glutin::init::<gfx::format::Rgba8, gfx::format::DepthStencil>(builder, &events);

            Box::new(ApplicationBase {
                platform: Platform::new(device, factory)
            })
        },
        PlatformIdentity::Metal => {
            use gfx::texture::Size;

            let events = winit::EventsLoop::new();
            let (window, mut device, mut factory, color) = gfx_window_metal::init::<gfx::format::Rgba8>(winit_builder, &events).unwrap();
            let (width, height) = window.get_inner_size_points().unwrap();
            // let _ = factory.create_depth_stencil_view_only(width as Size, height as Size).unwrap();

            Box::new(ApplicationBase {
                platform: Platform::new(device, factory)
            })
        },
        _ => panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
