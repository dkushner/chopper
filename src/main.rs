#[macro_use]
extern crate gfx;
extern crate gfx_core;
extern crate glutin;
extern crate nalgebra;
extern crate winit;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;

#[cfg(feature = "metal")]
extern crate gfx_device_metal;

#[cfg(feature = "metal")]
extern crate gfx_window_metal;

#[cfg(feature = "vulkan")]
extern crate gfx_device_vulkan;

#[cfg(feature = "vulkan")]
extern crate gfx_window_vulkan;

mod system;
mod core;

fn main() {
    println!("Hello, system!");
}
