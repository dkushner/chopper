use gfx::{self, Primitive, ShaderSet};
use std::collections::HashMap;
use gfx::shade;
use gfx::preset::depth::{LESS_EQUAL_TEST, LESS_EQUAL_WRITE, PASS_TEST};
use gfx::pso::PipelineState;
use gfx::state::{Depth, Rasterizer};
use core::platform::Platform;

use render::pipeline::{Meta, Data, Init};
use render::error::{RenderError, RenderResult};
use render::backend;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum ProgramSource {
    Minimal(&'static [u8], &'static [u8]),
    Geometry(&'static [u8], &'static [u8], &'static [u8]),
    Tessellation(&'static [u8], &'static [u8], &'static [u8])
}

impl ProgramSource {
    pub fn compile<B>(&self, platform: &mut Platform<B>) -> RenderResult<gfx::ShaderSet<B::Resources>> where B: backend::Backend {
        use gfx::Factory;
        use gfx::traits::FactoryExt;

        match *self {
            ProgramSource::Minimal(ref vertex_src, ref pixel_src) => {
                let vertex = platform.create_shader_vertex(vertex_src)
                    .map_err(|e| shade::ProgramError::Vertex(e));

                let pixel = platform.create_shader_pixel(pixel_src)
                    .map_err(|e| shade::ProgramError::Pixel(e));

                Ok(ShaderSet::Simple(vertex, pixel))
            },
            ProgramSource::Geometry(ref vertex_src, ref geometry_src, ref pixel_src) => {
                let vertex = platform.create_shader_vertex(vertex_src)
                    .map_err(|e| shade::ProgramError::Vertex(e));

                let geometry = platform.create_shader_geometry(geometry_src)
                    .map_err(|e| shade::ProgramError::Geometry(e));

                let pixel = platform.create_shader_pixel(pixel_src)
                    .map_err(|e| shade::ProgramError::Pixel(e));

                Ok(ShaderSet::Geometry(vertex, geometry, pixel))
            },
            ProgramSource::Tessellation(ref vertex_src, ref hull_src, ref domain_src, ref pixel_src) => {
                let vertex = platform.create_shader_vertex(vertex_src)
                    .map_err(|e| shade::ProgramError::Vertex(e));

                let hull = platform.create_shader_hull(hull_src)
                    .map_err(|e| shade::ProgramError::Hull(e));

                let domain = platform.create_shader_domain(domain_src)
                    .map_err(|e| shade::ProgramError::Domain(e));

                let pixel = platform.create_shader_pixel(pixel_src)
                    .map_err(|e| shade::ProgramError::Pixel(e));

                Ok(ShaderSet::Tessellated(vertex, hull, domain, pixel))
            }
        }
    }
}

/// Describes a rendering effect.
///
/// An effect is constituted by a pipeline state configuration along with the data necessary
/// to execute the pipeline.
#[derive(Derivative)]
#[derivative(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Effect<B: backend::Backend> {
    pipeline: PipelineState<B::Resources, Meta<B>>,
    #[derivative(Hash = "ignore")]
    pso_data: Data<B>,
    #[derivative(Hash = "ignore")]
    samplers: HashMap<String, B::Sampler>,
}

impl<B> Effect<B> where B: backend::Backend {
    pub fn new_minimal<S>(vertex_src: S, pixel_src: S) -> EffectBuilder<B>
        where S: Into<&'static [u8]> {

        EffectBuilder::new_minimal(vertex_src, pixel_src)
    }
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectBuilder<B: backend::Backend> {
    initializer: Init<'static, B>,
    depth_state: Depth,
    primitive: Primitive,
    program: ProgramSource,
    rasterizer: Rasterizer,
    samplers: HashMap<String, B::Sampler>,
}

impl<B> Default for EffectBuilder<B> where B: backend::Backend {
    fn default() -> Self {
        EffectBuilder {
            initializer: Init::default(),
            depth_state: LESS_EQUAL_WRITE,
            primitive: Primitive::TriangleList,
            program: ProgramSource::Simple("".as_bytes(), "".as_bytes()),
            rasterizer: Rasterizer::new_fill().with_cull_back(),
            samplers: HashMap::default()
        }
    }
}

impl<B> EffectBuilder<B> where B: backend::Backend {
    pub fn new_minimal<S>(vertex_src: S, pixel_src: S) -> EffectBuilder<B>
        where S: Into<&'static [u8]> {

        let (vertex_src, pixel_src) = (vertex_src.into(), pixel_src.into());

        EffectBuilder {
            program: ProgramSource::Minimal(vertex_src, pixel_src)
            .. Default::default()
        }
    }
}
