use gfx::{self, Device, Factory, Encoder};
use gfx_core::*;
use gfx_core::Resources;
use gfx_core::memory::Bind;
use gfx_core::factory::{ResourceViewError, TargetViewError};
use render::backend;

use gfx_device_gl;

#[cfg(feature = "metal")]
use gfx_device_metal;

#[cfg(feature = "vulkan")]
use gfx_device_vulkan;

pub enum PlatformIdentity {
    OpenGL,
    Metal,
    Vulkan
}

pub trait PlatformIdentifier {
    fn identify() -> PlatformIdentity;
}

impl PlatformIdentifier for Platform<backend::OpenGL> {
    fn identify() -> PlatformIdentity {
        PlatformIdentity::OpenGL
    }
}

#[cfg(feature = "metal")]
impl PlatformIdentifier for Platform<backend::Metal> {
    fn identify() -> PlatformIdentity {
        PlatformIdentity::Metal
    }
}

#[cfg(feature = "vulkan")]
impl PlatformIdentifier for Platform<backend::Vulkan> {
    fn identify() -> PlatformIdentity {
        PlatformIdentity::Vulkan
    }
}

pub struct Platform<B: backend::Backend> {
    device: B::Device,
    factory: B::Factory,
}

impl <B> Platform<B> where B: backend::Backend {
    pub fn new(device: B::Device, factory: B::Factory) -> Self {
        Platform {
            device,
            factory
        }
    }
}

impl <B> Device for Platform<B> where B: backend::Backend {
    type Resources = B::Resources;
    type CommandBuffer = B::CommandBuffer;

    fn get_capabilities(&self) -> &Capabilities {
        self.device.get_capabilities()
    }

    fn pin_submitted_resources(&mut self, handle: &handle::Manager<Self::Resources>) {
        self.device.pin_submitted_resources(handle)
    }

    fn submit(&mut self, commands: &mut Self::CommandBuffer, access: &command::AccessInfo<Self::Resources>) -> SubmissionResult<()> {
        self.device.submit(commands, access)
    }

    fn fenced_submit(&mut self, commands: &mut Self::CommandBuffer, access: &command::AccessInfo<Self::Resources>, after: Option<handle::Fence<Self::Resources>>) -> SubmissionResult<handle::Fence<Self::Resources>> {
        self.device.fenced_submit(commands, access, after)
    }

    fn wait_fence(&mut self, resources: &handle::Fence<Self::Resources>) {
        self.device.wait_fence(resources)
    }

    fn cleanup(&mut self) {
        self.device.cleanup()
    }
}

impl <B> Factory<B::Resources> for Platform<B> where B: backend::Backend {
    fn get_capabilities(&self) -> &Capabilities {
        self.device.get_capabilities()
    }

    fn create_buffer_raw(&mut self, info: buffer::Info) -> Result<handle::RawBuffer<B::Resources>, buffer::CreationError> {
        self.factory.create_buffer_raw(info)
    }

    fn create_buffer_immutable_raw(&mut self, data: &[u8], stride: usize, role: buffer::Role, bind: Bind) -> Result<handle::RawBuffer<B::Resources>, buffer::CreationError> {
        self.factory.create_buffer_immutable_raw(data, stride, role, bind)
    }

    fn create_pipeline_state_raw(&mut self, handle: &handle::Program<B::Resources>, descriptor: &pso::Descriptor) -> Result<handle::RawPipelineState<B::Resources>, pso::CreationError> {
        self.factory.create_pipeline_state_raw(handle, descriptor)
    }

    fn create_program(&mut self, shader_set: &ShaderSet<B::Resources>) -> Result<handle::Program<B::Resources>, shade::CreateProgramError> {
        self.factory.create_program(shader_set)
    }

    fn create_shader(&mut self, stage: shade::Stage, code: &[u8]) -> Result<handle::Shader<B::Resources>, shade::CreateShaderError> {
        self.factory.create_shader(stage, code)

    }

    fn create_sampler(&mut self, sampler_info: texture::SamplerInfo) -> handle::Sampler<B::Resources> {
        self.factory.create_sampler(sampler_info)
    }

    fn read_mapping<'a, 'b, T>(&'a mut self, buf: &'b handle::Buffer<B::Resources, T>) -> Result<mapping::Reader<'b, B::Resources, T>, mapping::Error> where T: Copy {
        self.factory.read_mapping(buf)
    }

    fn write_mapping<'a, 'b, T>(&'a mut self, buf: &'b handle::Buffer<B::Resources, T>) -> Result<mapping::Writer<'b, B::Resources, T>, mapping::Error> where T: Copy {
        self.factory.write_mapping(buf)
    }

    fn create_texture_raw(&mut self, info: texture::Info, channel: Option<format::ChannelType>, data: Option<&[&[u8]]>) -> Result<handle::RawTexture<B::Resources>, texture::CreationError> {
        self.factory.create_texture_raw(info, channel, data)
    }

    fn view_buffer_as_shader_resource_raw(&mut self, handle: &handle::RawBuffer<B::Resources>) -> Result<handle::RawShaderResourceView<B::Resources>, ResourceViewError> {
        self.factory.view_buffer_as_shader_resource_raw(handle)
    }

    fn view_buffer_as_unordered_access_raw(&mut self, handle: &handle::RawBuffer<B::Resources>) -> Result<handle::RawUnorderedAccessView<B::Resources>, ResourceViewError> {
        self.factory.view_buffer_as_unordered_access_raw(handle)
    }

    fn view_texture_as_shader_resource_raw(&mut self, handle: &handle::RawTexture<B::Resources>, description: texture::ResourceDesc) -> Result<handle::RawShaderResourceView<B::Resources>, ResourceViewError> {
        self.factory.view_texture_as_shader_resource_raw(handle, description)
    }

    fn view_texture_as_unordered_access_raw(&mut self, handle: &handle::RawTexture<B::Resources>) -> Result<handle::RawUnorderedAccessView<B::Resources>, ResourceViewError> {
        self.factory.view_texture_as_unordered_access_raw(handle)
    }

    fn view_texture_as_render_target_raw(&mut self, handle: &handle::RawTexture<B::Resources>, description: texture::RenderDesc) -> Result<handle::RawRenderTargetView<B::Resources>, TargetViewError> {
        self.factory.view_texture_as_render_target_raw(handle, description)
    }

    fn view_texture_as_depth_stencil_raw(&mut self, handle: &handle::RawTexture<B::Resources>, description: texture::DepthStencilDesc) -> Result<handle::RawDepthStencilView<B::Resources>, TargetViewError> {
        self.factory.view_texture_as_depth_stencil_raw(handle, description)
    }
}
