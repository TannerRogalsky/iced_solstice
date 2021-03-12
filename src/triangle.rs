//! Draw meshes of triangles.
use crate::program;
use crate::Transformation;
use iced_graphics::layer;

pub use iced_graphics::triangle::{Mesh2D, Vertex2D};
use solstice::mesh::IndexedMesh;
use solstice::shader::{DynamicShader, RawUniformValue, UniformLocation};
use solstice::vertex::VertexFormat;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct Vertex(Vertex2D);

unsafe impl bytemuck::Zeroable for Vertex {}
unsafe impl bytemuck::Pod for Vertex {}

impl solstice::vertex::Vertex for Vertex {
    fn build_bindings() -> &'static [VertexFormat] {
        &[
            VertexFormat {
                name: "i_Position",
                offset: 0,
                atype: solstice::vertex::AttributeType::F32F32,
                normalize: false,
            },
            VertexFormat {
                name: "i_Color",
                offset: std::mem::size_of::<[f32; 2]>(),
                atype: solstice::vertex::AttributeType::F32F32F32F32,
                normalize: false,
            },
        ]
    }
}

const VERTEX_BUFFER_SIZE: usize = 10_000;
const INDEX_BUFFER_SIZE: usize = 10_000;

#[derive(Debug)]
pub(crate) struct Pipeline {
    program: DynamicShader,
    mesh: IndexedMesh<Vertex, u32>,
    transform_location: UniformLocation,
    current_transform: Transformation,
}

impl Pipeline {
    pub fn new(gl: &mut solstice::Context) -> Pipeline {
        let program = {
            const SRC: &str = include_str!("shader/triangle.glsl");
            program::create(gl, SRC, SRC)
        };

        let transform_location = program
            .get_uniform_by_name("u_Transform")
            .unwrap()
            .location
            .clone();

        gl.use_shader(Some(&program));
        let transform: [f32; 16] = Transformation::identity().into();
        gl.set_uniform_by_location(
            &transform_location,
            &RawUniformValue::Mat4(transform.into()),
        );

        let mesh = IndexedMesh::new(gl, VERTEX_BUFFER_SIZE, INDEX_BUFFER_SIZE).unwrap();

        Pipeline {
            program,
            mesh,
            transform_location,
            current_transform: Transformation::identity(),
        }
    }

    pub fn draw(
        &mut self,
        gl: &mut solstice::Context,
        target_height: u32,
        transformation: Transformation,
        scale_factor: f32,
        meshes: &[layer::Mesh<'_>],
    ) {
        // We upload all the vertices and indices upfront
        let mut last_vertex = 0;
        let mut last_index = 0;

        let mut index_scratch = Vec::new();

        for layer::Mesh { buffers, .. } in meshes {
            let vertices = bytemuck::cast_slice(buffers.vertices.as_slice());

            index_scratch.clear();
            for index in buffers.indices.iter() {
                index_scratch.push(index + last_vertex as u32)
            }

            self.mesh.set_vertices(gl, vertices, last_vertex);
            self.mesh.set_indices(gl, &index_scratch, last_index);

            last_vertex += buffers.vertices.len();
            last_index += buffers.indices.len();
        }

        // Then we draw each mesh using offsets
        let mut last_index = 0;

        for layer::Mesh {
            buffers,
            origin,
            clip_bounds,
        } in meshes
        {
            let transform = transformation * Transformation::translate(origin.x, origin.y);
            if self.current_transform != transform {
                gl.use_shader(Some(&self.program));
                let matrix: [f32; 16] = transform.into();
                gl.set_uniform_by_location(
                    &self.transform_location,
                    &RawUniformValue::Mat4(matrix.into()),
                );

                self.current_transform = transform;
            }

            let clip_bounds = (*clip_bounds * scale_factor).snap();
            let scissor = solstice::viewport::Viewport::new(
                clip_bounds.x as i32,
                (target_height - (clip_bounds.y + clip_bounds.height)) as i32,
                clip_bounds.width as i32,
                clip_bounds.height as i32,
            );

            let offset = last_index * std::mem::size_of::<u32>();
            let geometry = solstice::Geometry {
                mesh: &self.mesh,
                draw_range: offset..(offset + buffers.indices.len()),
                draw_mode: solstice::DrawMode::Triangles,
                instance_count: 1,
            };

            solstice::Renderer::draw(
                gl,
                &self.program,
                &geometry,
                solstice::PipelineSettings {
                    polygon_state: Default::default(),
                    depth_state: None,
                    scissor_state: Some(scissor),
                    ..Default::default()
                },
            );

            last_index += buffers.indices.len();
        }
    }
}
