use gfx;
use gfx_core;
use glutin;
use gfx_device_gl;
use gfx_window_glutin;

#[cfg(feature = "metal")]
use gfx_device_metal;

#[cfg(feature = "metal")]
use gfx_window_metal;

#[cfg(feature = "vulkan")]
use gfx_device_vulkan;

#[cfg(feature = "vulkan")]
use gfx_window_vulkan;

/// OpenGL rendering backend.
pub struct OpenGL;

/// Metal rendering backend.
pub struct Metal;

/// Vulkan rendering backend.
pub struct Vulkan;

pub trait Backend {
    type Resources: gfx::Resources;
    type Device: gfx::Device;
    type Factory: gfx::Factory<Self::Resources>;
    type CommandBuffer: gfx::CommandBuffer<Self::Resources>;
    type ColorFormat: gfx::format::Formatted;
    type DepthFormat: gfx::format::Formatted;
    type Window;
    type ShaderModel;
    type DepthStencilView;
    type RenderTargetView;
    type Sampler;
}

impl Backend for OpenGL where {
    type Resources = gfx_device_gl::Resources;
    type Device = gfx_device_gl::Device;
    type Factory = gfx_device_gl::Factory;
    type CommandBuffer = gfx_device_gl::CommandBuffer;
    type ColorFormat = gfx::format::Srgba8;
    type DepthFormat = gfx::format::DepthStencil;
    type Window = glutin::Window;
    type ShaderModel = gfx_device_gl::Version;
    type DepthStencilView = gfx::handle::DepthStencilView<gfx_device_gl::Resources, Self::DepthFormat>;
    type RenderTargetView = gfx::handle::RenderTargetView<gfx_device_gl::Resources, Self::ColorFormat>;
    type Sampler = gfx::handle::Sampler<gfx_device_gl::Resources>;
}

#[cfg(feature = "metal")]
impl Backend for Metal {
    type Resources = gfx_device_metal::Resources;
    type Device = gfx_device_metal::Device;
    type Factory = gfx_device_metal::Factory;
    type CommandBuffer = gfx_device_metal::CommandBuffer;
    type ColorFormat = gfx::format::Srgba8;
    type DepthFormat = gfx::format::Depth32F;
    type Window = gfx_window_metal::MetalWindow;
    type ShaderModel = ();
    type DepthStencilView = gfx::handle::DepthStencilView<gfx_device_metal::Resources, Self::DepthFormat>;
    type RenderTargetView = gfx::handle::RenderTargetView<gfx_device_metal::Resources, Self::ColorFormat>;
    type Sampler = gfx::handle::Sampler<gfx_device_metal::Resources>;
}

#[cfg(feature = "vulkan")]
impl Backend for Vulkan {
    type Resources = gfx_device_vulkan::Resources;
    type Device = gfx_device_vulkan::Device;
    type Factory = gfx_device_vulkan::Factory;
    type CommandBuffer = gfx_device_vulkan::CommandBuffer;
    type ColorFormat = gfx::format::Srgba8;
    type DepthFormat = gfx::format::DepthStencil;
    type Window = gfx_window_vulkan::Window;
    type ShaderModel = ();
    type DepthStencilView = gfx::handle::DepthStencilView<gfx_device_vulkan::Resources, Self::DepthFormat>;
    type RenderTargetView = gfx::handle::RenderTargetView<gfx_device_vulkan::Resources, Self::ColorFormat>;
    type Sampler = gfx::handle::Sampler<gfx_device_vulkan::Resources>;
}


