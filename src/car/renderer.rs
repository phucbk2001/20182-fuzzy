use nalgebra as na;

use crate::bezier;
use crate::config::Config;
use crate::glhelper;
use crate::car::CarSystem;

use image;

use glium::implement_vertex;
use glium::uniform;
use glium::{
    Program, Display, 
    Surface,
};
use glium::texture::Texture2d;

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

type VertexBuffer = glium::VertexBuffer<Vertex>;
type IndexBuffer = glium::IndexBuffer<u16>;

pub struct CarRenderer {
    vertex_buffers: Vec<VertexBuffer>,
    index_buffers: Vec<IndexBuffer>,
    textures: Vec<Texture2d>,
    program: Program,
}

impl CarRenderer {
    pub fn new(display: &Display, config: &Config) -> Self {
        let program = glium::Program::from_source(
            display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();

        let width = config.car_width;
        let length = config.car_length;

        let vertices = vec![
            Vertex { position: [length/2.0, width/2.0], tex_coords: [0.0, 0.0] },
            Vertex { position: [-length/2.0, width/2.0], tex_coords: [1.0, 0.0] },
            Vertex { position: [-length/2.0, -width/2.0], tex_coords: [1.0, 1.0] },
            Vertex { position: [length/2.0, -width/2.0], tex_coords: [0.0, 1.0] },
        ];

        let vertex_buffer = VertexBuffer::new(
            display, 
            &vertices,
        ).unwrap();

        let indices: Vec<u16> = vec![0, 1, 2, 2, 3, 0];

        let index_buffer = IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        ).unwrap();

        let texture = glhelper::load_texture(
            "assets/car1.png",
            image::ImageFormat::PNG,
            display
        );

        Self {
            vertex_buffers: vec![vertex_buffer],
            index_buffers: vec![index_buffer],
            textures: vec![texture],
            program: program,
        }
    }

    pub fn render<T>(
        &self, target: &mut T, car_system: &CarSystem,
        view_proj: &na::Matrix4<f32>) 
        where T: Surface
    {
        use glium::draw_parameters::DrawParameters;
        let mut params: DrawParameters = Default::default();

        let blend = glium::Blend {
            color: glium::BlendingFunction::Addition {
                source: glium::LinearBlendingFactor::SourceAlpha, 
                destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
            },
            alpha: glium::BlendingFunction::Addition {
                source: glium::LinearBlendingFactor::SourceAlpha, 
                destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
            },
            constant_value: (0.0, 0.0, 0.0, 0.0),
        };

        params.blend = blend;

        for (e, car) in car_system.cars.iter() {
            if car_system.em.is_alive(*e) {
                let world = bezier::Matrix::rotation_from(
                    car.direction).to_na_matrix()
                    .append_translation(&na::Vector3::new(
                        car.position.x, car.position.y, 0.0));
                let matrix = *view_proj * world;
                let matrix_ref: &[[f32; 4]; 4] = matrix.as_ref();

                let uniform = uniform! {
                    matrix: *matrix_ref,
                    tex: &self.textures[0],
                };

                target.draw(
                    &self.vertex_buffers[0],
                    &self.index_buffers[0],
                    &self.program,
                    &uniform, 
                    &params).unwrap();
            }
        }

    }
}
