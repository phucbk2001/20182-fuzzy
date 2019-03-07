use nalgebra as na;

use glium::implement_vertex;
use glium::uniform;
use glium::{
    Program, Display, 
    VertexBuffer, IndexBuffer,
    Surface,
};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

const VERTEX_SHADER_SRC: &'static str = r#"
    #version 140

    in vec2 position;
    in vec2 tex_coords;

    out vec2 texture_coords;

    uniform mat4 matrix;

    void main() {
        texture_coords = tex_coords;
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER_SRC: &'static str = r#"
    #version 140
    in vec2 texture_coords;

    out vec4 color;

    uniform sampler2D tex;

    void main() {
        color = texture(tex, texture_coords);
    }
"#;

pub struct CarRenderer {
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    program: Program,
}

impl CarRenderer {
    pub fn new(display: &Display) -> Self {
        let program = glium::Program::from_source(
            display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();

        Self {
            vertex_buffer: VertexBuffer::empty(display, 0).unwrap(),
            index_buffer: IndexBuffer::empty(
                display,
                glium::index::PrimitiveType::TrianglesList,
                0
            ).unwrap(),
            program: program,
        }
    }

    pub fn render<T>(&self, target: &mut T, matrix: &na::Matrix4<f32>) 
        where T: Surface
    {
        use glium::draw_parameters::DrawParameters;
        let mut params: DrawParameters = Default::default();

        let matrix_ref: &[[f32; 4]; 4] = matrix.as_ref();

        let uniform = uniform! {
            matrix: *matrix_ref,
        };
        target.draw(
            &self.vertex_buffer,
            &self.index_buffer,
            &self.program,
            &uniform, 
            &params).unwrap();
    }
}
