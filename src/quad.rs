use crate::program;
use crate::Transformation;
use bytemuck::{Pod, Zeroable};
use iced_graphics::layer;
use iced_native::Rectangle;
use solstice::{
    mesh::VertexMesh,
    shader::{DynamicShader, RawUniformValue, UniformLocation},
    vertex::Vertex,
};

const MAX_INSTANCES: usize = 100_000;

#[derive(Copy, Clone, Debug, Vertex, Pod, Zeroable)]
#[repr(C)]
struct Position {
    position: [f32; 2],
}

#[derive(Debug)]
pub struct Pipeline {
    program: DynamicShader,
    quad: VertexMesh<Position>,
    instances: VertexMesh<Quad>,
    transform_location: UniformLocation,
    scale_location: UniformLocation,
    screen_height_location: UniformLocation,
    current_transform: Transformation,
    current_scale: f32,
    current_target_height: u32,
}

impl Pipeline {
    pub fn new(gl: &mut solstice::Context) -> Pipeline {
        let program = {
            const SRC: &str = include_str!("shader/quad.glsl");
            program::create(gl, SRC, SRC)
        };

        let transform_location = program
            .get_uniform_by_name("u_Transform")
            .unwrap()
            .location
            .clone();
        let scale_location = program
            .get_uniform_by_name("u_Scale")
            .unwrap()
            .location
            .clone();
        let screen_height_location = program
            .get_uniform_by_name("u_ScreenHeight")
            .unwrap()
            .location
            .clone();

        gl.use_shader(Some(&program));
        let matrix: [f32; 16] = Transformation::identity().into();
        gl.set_uniform_by_location(&transform_location, &RawUniformValue::Mat4(matrix.into()));
        gl.set_uniform_by_location(&scale_location, &RawUniformValue::Float(1.0));
        gl.set_uniform_by_location(&screen_height_location, &RawUniformValue::Float(0.));

        let instances = VertexMesh::new(gl, MAX_INSTANCES).unwrap();
        let quad = VertexMesh::with_data(
            gl,
            &[
                Position { position: [0., 0.] },
                Position { position: [0., 1.] },
                Position { position: [1., 0.] },
                Position { position: [1., 1.] },
            ],
        )
        .unwrap();

        Pipeline {
            program,
            quad,
            instances,
            transform_location,
            scale_location,
            screen_height_location,
            current_transform: Transformation::identity(),
            current_scale: 1.0,
            current_target_height: 0,
        }
    }

    pub fn draw(
        &mut self,
        gl: &mut solstice::Context,
        target_height: u32,
        instances: &[layer::Quad],
        transformation: Transformation,
        scale: f32,
        bounds: Rectangle<u32>,
    ) {
        let scissor = solstice::viewport::Viewport::new(
            bounds.x as i32,
            (target_height - (bounds.y + bounds.height)) as i32,
            bounds.width as i32,
            bounds.height as i32,
        );

        gl.use_shader(Some(&self.program));

        if transformation != self.current_transform {
            let matrix: [f32; 16] = transformation.into();
            gl.set_uniform_by_location(
                &self.transform_location,
                &RawUniformValue::Mat4(matrix.into()),
            );

            self.current_transform = transformation;
        }

        if scale != self.current_scale {
            gl.set_uniform_by_location(&self.scale_location, &RawUniformValue::Float(scale));

            self.current_scale = scale;
        }

        if target_height != self.current_target_height {
            gl.set_uniform_by_location(
                &self.screen_height_location,
                &RawUniformValue::Float(target_height as f32),
            );

            self.current_target_height = target_height;
        }

        let mut i = 0;
        let total = instances.len();

        while i < total {
            let end = (i + MAX_INSTANCES).min(total);
            let amount = end - i;

            self.instances
                .set_vertices(gl, bytemuck::cast_slice(&instances[i..end]), 0);
            use solstice::mesh::MeshAttacher;
            let attached = self.quad.attach_with_step(&self.instances, 1);

            let geometry = solstice::Geometry {
                mesh: attached,
                draw_range: 0..4,
                draw_mode: solstice::DrawMode::TriangleStrip,
                instance_count: amount as _,
            };
            solstice::Renderer::draw(
                gl,
                &self.program,
                &geometry,
                solstice::PipelineSettings {
                    depth_state: None,
                    scissor_state: Some(scissor),
                    ..Default::default()
                },
            );

            i += MAX_INSTANCES;
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct Quad(layer::Quad);

unsafe impl bytemuck::Zeroable for Quad {}
unsafe impl bytemuck::Pod for Quad {}

impl Default for Quad {
    fn default() -> Self {
        Quad(layer::Quad::zeroed())
    }
}

impl solstice::vertex::Vertex for Quad {
    fn build_bindings() -> &'static [solstice::vertex::VertexFormat] {
        use solstice::vertex::{AttributeType, VertexFormat};
        &[
            VertexFormat {
                name: "i_Pos",
                offset: 0,
                atype: AttributeType::F32F32,
                normalize: false,
            },
            VertexFormat {
                name: "i_Scale",
                offset: std::mem::size_of::<[f32; 2]>(),
                atype: AttributeType::F32F32,
                normalize: false,
            },
            VertexFormat {
                name: "i_Color",
                offset: std::mem::size_of::<[f32; 4]>(),
                atype: AttributeType::F32F32F32F32,
                normalize: false,
            },
            VertexFormat {
                name: "i_BorderColor",
                offset: std::mem::size_of::<[f32; 8]>(),
                atype: AttributeType::F32F32F32F32,
                normalize: false,
            },
            VertexFormat {
                name: "i_BorderRadius",
                offset: std::mem::size_of::<[f32; 12]>(),
                atype: AttributeType::F32,
                normalize: false,
            },
            VertexFormat {
                name: "i_BorderWidth",
                offset: std::mem::size_of::<[f32; 13]>(),
                atype: AttributeType::F32,
                normalize: false,
            },
        ]
    }
}

// unsafe fn create_instance_buffer(
//     gl: &mut solstice::Context,
//     size: usize,
// ) -> (
//     <glow::Context as HasContext>::VertexArray,
//     <glow::Context as HasContext>::Buffer,
// ) {
//     let vertex_array = gl.create_vertex_array().expect("Create vertex array");
//     let buffer = gl.create_buffer().expect("Create instance buffer");
//
//     gl.bind_vertex_array(Some(vertex_array));
//     gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));
//     gl.buffer_data_size(
//         glow::ARRAY_BUFFER,
//         (size * std::mem::size_of::<layer::Quad>()) as i32,
//         glow::DYNAMIC_DRAW,
//     );
//
//     let stride = std::mem::size_of::<layer::Quad>() as i32;
//
//     gl.enable_vertex_attrib_array(0);
//     gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, stride, 0);
//     gl.vertex_attrib_divisor(0, 1);
//
//     gl.enable_vertex_attrib_array(1);
//     gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, stride, 4 * 2);
//     gl.vertex_attrib_divisor(1, 1);
//
//     gl.enable_vertex_attrib_array(2);
//     gl.vertex_attrib_pointer_f32(2, 4, glow::FLOAT, false, stride, 4 * (2 + 2));
//     gl.vertex_attrib_divisor(2, 1);
//
//     gl.enable_vertex_attrib_array(3);
//     gl.vertex_attrib_pointer_f32(3, 4, glow::FLOAT, false, stride, 4 * (2 + 2 + 4));
//     gl.vertex_attrib_divisor(3, 1);
//
//     gl.enable_vertex_attrib_array(4);
//     gl.vertex_attrib_pointer_f32(4, 1, glow::FLOAT, false, stride, 4 * (2 + 2 + 4 + 4));
//     gl.vertex_attrib_divisor(4, 1);
//
//     gl.enable_vertex_attrib_array(5);
//     gl.vertex_attrib_pointer_f32(5, 1, glow::FLOAT, false, stride, 4 * (2 + 2 + 4 + 4 + 1));
//     gl.vertex_attrib_divisor(5, 1);
//
//     gl.bind_vertex_array(None);
//     gl.bind_buffer(glow::ARRAY_BUFFER, None);
//
//     (vertex_array, buffer)
// }
