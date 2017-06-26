use system::entity::Entity;

gfx_defines!{

}

pub type Mesh = u32;

pub struct RenderManager {
    meshes: BTreeMap<Entity, Mesh>,

    entity: Vec<Entity>,
}

mod particle {
    gfx_defines! {
        vertex Vertex {
            position: [f32; 3] = "position",
            velocity: [f32; 3] = "velocity",
            color: [f32; 4] = "color",
        }

        constant Locals {
            aspect: f32 = "aspect"
        }

        pipeline pipe {
            vertex_buffer: gfx::VertexBuffer<Vertex> = (),
            locals: gfx::ConstantBuffer<Locals> = "Locals",
            out_color: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
        }
    }

    impl Vertex {
        fn new() -> Self {
            Vertex {
                position: Default::default(),
                velocity: Default::default(),
                color: [f32; 4] = "color",
            }
        }
    }
}
