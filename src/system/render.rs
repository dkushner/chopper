use gfx;
use system::entity::Entity;
use std::collections::BTreeMap;
use nalgebra::Vector3;

pub type Mesh = u32;

mod mesh {
    use gfx;

    gfx_defines!{
        vertex Vertex {
            position: [f32; 3] = "position",
            uv: [f32; 2] = "uv",
        }

        vertex Instance {
            translate: [f32; 3] = "translate",
            color: u32 = "color",
        }

        constant Locals {
            transform: [[f32; 4]; 4] = "transform",
        }

        pipeline pipe {
            vertices: gfx::VertexBuffer<Vertex> = (),
            vertex_locals: gfx::ConstantBuffer<Locals> = "locals",
        }
    }
}

pub struct MeshDescription {
    pub vertices: Vector3<f32>,
    pub uvs: Vector3<f32>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {

    }
}
