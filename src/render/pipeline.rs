use gfx;
use gfx_core;
use gfx::pso::{PipelineData, PipelineInit, DataBind, DataLink};
use gfx::pso::{self, target};
use render::backend;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Meta<R: backend::Backend> {
    constants: Vec<gfx::RawConstantBuffer>,
    globals: Vec<gfx::RawGlobal>,
    color_targets: Vec<gfx::RenderTarget<R::ColorFormat>>,
    depth_target: Option<gfx::DepthStencilTarget<R::DepthFormat>>,
    samplers: Vec<gfx::Sampler>,
    textures: Vec<gfx::RawShaderResource>,
    vertices: Vec<gfx::RawVertexBuffer>,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Init<'d, R> where R: backend::Backend,
    <R::ColorFormat as gfx_core::format::Formatted>::Channel: gfx_core::format::RenderChannel,
    <R::ColorFormat as gfx_core::format::Formatted>::Surface: gfx_core::format::RenderSurface,
    <R::DepthFormat as gfx_core::format::Formatted>::Channel: gfx_core::format::RenderChannel,
    <R::DepthFormat as gfx_core::format::Formatted>::Surface: gfx_core::format::DepthSurface,
    <R::DepthFormat as gfx_core::format::Formatted>::Surface: gfx_core::format::StencilSurface {

    pub constants: Vec<<gfx::RawConstantBuffer as DataLink<'d>>::Init>,
    pub globals: Vec<<gfx::RawGlobal as DataLink<'d>>::Init>,
    pub color_targets: Vec<<target::RenderTarget<R::ColorFormat> as DataLink<'d>>::Init>,
    pub depth_targets: Option<<target::DepthStencilTarget<R::DepthFormat> as DataLink<'d>>::Init>,
    pub samplers: Vec<<gfx::Sampler as DataLink<'d>>::Init>,
    pub textures: Vec<<gfx::RawShaderResource as DataLink<'d>>::Init>,
    pub vertices: Vec<<gfx::RawVertexBuffer as DataLink<'d>>::Init>,
}

impl<'d, R> PipelineInit for Init<'d, R> where R: backend::Backend,
    <R::ColorFormat as gfx_core::format::Formatted>::Channel: gfx_core::format::RenderChannel,
    <R::ColorFormat as gfx_core::format::Formatted>::Surface: gfx_core::format::RenderSurface,
    <R::DepthFormat as gfx_core::format::Formatted>::Channel: gfx_core::format::RenderChannel,
    <R::DepthFormat as gfx_core::format::Formatted>::Surface: gfx_core::format::DepthSurface,
    <R::DepthFormat as gfx_core::format::Formatted>::Surface: gfx_core::format::StencilSurface {

    type Meta = Meta<R>;

    fn link_to<'s>(&self, desc: &mut pso::Descriptor, info: &'s gfx::ProgramInfo) -> Result<Self::Meta, pso::InitError<&'s str>> {
        let mut meta = Meta::default();

        for (info, buffer) in info.constant_buffers.iter().zip(&self.constants) {
            let mut meta_constants = <gfx::RawConstantBuffer as DataLink<'d>>::new();
            if let Some(res) = meta_constants.link_constant_buffer(info, buffer) {
                let d = res.map_err(|e| pso::InitError::ConstantBuffer(info.name.as_str(), Some(e)))?;
                meta.constants.push(meta_constants);
                desc.constant_buffers[info.slot as usize] = Some(d);
            }
        }

        for (info, global) in info.globals.iter().zip(&self.globals) {
            let mut meta_globals = <gfx::RawGlobal as DataLink<'d>>::new();
            if let Some(res) = meta_globals.link_global_constant(info, global) {
                res.map_err(|e| pso::InitError::GlobalConstant(info.name.as_str(), Some(e)))?;
                meta.globals.push(meta_globals);
            }
        }

        for (info, color) in info.outputs.iter().zip(&self.out_colors) {
            let mut meta_color_targets = <target::RenderTarget<R::ColorFormat> as DataLink<'d>>::new();
            if let Some(res) = meta_color_targets.link_output(info, color) {
                let d = res.map_err(|e| pso::InitError::PixelExport(info.name.as_str(), Some(e)))?;
                meta.color_targets.push(meta_color_targets);
                desc.color_targets[info.slot as usize] = Some(d);
            }
        }

        if let Some(depth) = self.out_depth {
            let mut meta_depth_target = <target::DepthStencilTarget<R::DepthFormat> as DataLink<'d>>::new();
            if let Some(d) = meta_depth_target.link_depth_stencil(&depth) {
                desc.scissor = meta_depth_target.link_scissor();
                meta.out_depth = Some(meta_depth_target);
                desc.depth_stencil = Some(d);
            }
        }

        for (info, smp) in info.samplers.iter().zip(&self.samplers) {
            let mut meta_samplers = <gfx::Sampler as DataLink<'d>>::new();
            if let Some(d) = meta_samplers.link_sampler(info, smp) {
                meta.samplers.push(meta_samplers);
                desc.samplers[info.slot as usize] = Some(d);
            }
        }

        for (info, tex) in info.textures.iter().zip(&self.textures) {
            let mut meta_textures = <gfx::RawShaderResource as DataLink<'d>>::new();
            if let Some(res) = meta_textures.link_resource_view(info, tex) {
                let d = res.map_err(|_| pso::InitError::ResourceView(info.name.as_str(), Some(())))?;
                meta.textures.push(meta_textures);
                desc.resource_views[info.slot as usize] = Some(d);
            }
        }

        for (i, vbuf) in self.vertex_bufs.iter().enumerate() {
            let mut meta_vertices = <gfx::RawVertexBuffer as DataLink<'d>>::new();
            if let Some(d) = meta_vertices.link_vertex_buffer(i as u8, vbuf) {
                for attr in info.vertex_attributes.iter() {
                    if let Some(res) = meta_vertices.link_input(attr, vbuf) {
                        let d = res.map_err(|e| pso::InitError::VertexImport(attr.name.as_str(), Some(e)))?;
                        desc.attributes[attr.slot as usize] = Some(d);
                    }
                }

                meta.vertices.push(meta_vertices);
                desc.vertex_buffers[i] = Some(d);
            }
        }

        Ok(meta)
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Data<B: backend::Backend> {
    pub constants: Vec<<gfx::RawConstantBuffer as DataBind<B::Resources>>::Data>,
    pub globals: Vec<<gfx::RawGlobal as DataBind<B::Resources>>::Data>,
    pub color_targets: Vec<<gfx::RenderTarget<B::ColorFormat> as DataBind<B::Resources>>::Data>,
    pub depth_target: Option<<gfx::DepthStencilTarget<B::DepthFormat> as DataBind<B::Resources>>::Data>,
    pub samplers: Vec<<gfx::Sampler as DataBind<B::Resources>>::Data>,
    pub textures: Vec<<gfx::RawShaderResource as DataBind<B::Resources>>::Data>,
    pub vertices: Vec<<gfx::RawVertexBuffer as DataBind<B::Resources>>::Data>,
}

impl<B> PipelineData<B::Resources> for Data<B> where B: backend::Backend {
    type Meta = Meta<B>;

    fn bake_to(&self, out: &mut gfx::pso::RawDataSet<B::Resources>, meta: &Self::Meta, manager: &mut gfx::handle::Manager<B::Resources>, access: &mut gfx::pso::AccessInfo<B::Resources>) {
        let constants = meta.const_bufs.iter().zip(&self.constants);
        for (meta_buffer, buffer) in constants {
            meta_buffer.bind_to(out, &buffer, manager, access);
        }

        let globals = meta.globals.iter().zip(&self.globals);
        for (meta_global, global) in globals {
            meta_global.bind_to(out, &global, manager, access);
        }

        let color_targets = meta.color_targets.iter().zip(&self.color_targets);
        for (meta_target, target) in color_targets {
            meta_target.bind_to(out, &target, manager, access);
        }

        let depth_target = (meta.depth_target.as_ref(), self.depth_target.as_ref());
        if let (Some(ref meta_target), Some(ref target)) = depth_target {
            meta_target.bind_to(out, &target, manager, access);
        }

        let samplers = meta.samplers.iter().zip(&self.samplers);
        for (meta_sampler, sampler) in samplers {
            meta_sampler.bind_to(out, &sampler, manager, access);
        }

        let textures = meta.textures.iter().zip(&self.textures);
        for (meta_texture, texture) in textures {
            meta_texture.bind_to(out, &texture, manager, access);
        }

        let vertices = meta.vertex_bufs.iter().zip(&self.vertex_bufs);
        for (meta_buffer, buffer) in vertices {
            meta_buffer.bind_to(out, &buffer, manager, access);
        }
    }
}
