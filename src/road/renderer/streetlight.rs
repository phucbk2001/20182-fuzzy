use nalgebra as na;
use crate::config::Config;
use crate::glhelper;
use super::super::{Road, Lane, LaneId, Color};
use crate::bezier::Point;

use glium::implement_vertex;
use glium::uniform;
use glium::{
    Program, Display, 
    Surface,
};
use glium::texture::Texture2d;

#[derive(Copy, Clone)]
pub struct Vertex {
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

pub struct StreetLight {
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    textures: Vec<Texture2d>,
    program: Program,
    streetlights: Vec<(LaneId, Point)>,
}

const RED_INDEX: usize = 0;
const GREEN_INDEX: usize = 1;
const YELLOW_INDEX: usize = 2;

fn lane_to_streetlight_position(
    road: &Road, lane: &Lane, distance: f32) 
    -> Point 
{
    let left = *lane.left.iter().last()
        .expect("streetlight: Can't have empty left");
    let right = *lane.right.iter().last()
        .expect("streetlight: Can't have empty right");

    let left_pos = road.get_bezier(left).pos(1.0);
    let right_pos = road.get_bezier(right).pos(1.0);
    let v = (right_pos - left_pos).normalize();

    right_pos + v * distance
}

impl StreetLight {
    pub fn new(display: &Display, road: &Road, config: &Config) -> Self {
        let program = glium::Program::from_source(
            display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();

        let size = config.streetlight_size;

        let vertices = vec![
            Vertex { position: [size/2.0, size/2.0], tex_coords: [1.0, 0.0] },
            Vertex { position: [-size/2.0, size/2.0], tex_coords: [0.0, 0.0] },
            Vertex { position: [-size/2.0, -size/2.0], tex_coords: [0.0, 1.0] },
            Vertex { position: [size/2.0, -size/2.0], tex_coords: [1.0, 1.0] },
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

        let red = glhelper::load_texture(
            "assets/red.png",
            image::ImageFormat::PNG,
            display
        );

        let green = glhelper::load_texture(
            "assets/green.png",
            image::ImageFormat::PNG,
            display
        );

        let yellow = glhelper::load_texture(
            "assets/yellow.png",
            image::ImageFormat::PNG,
            display
        );

        let streetlights: Vec<(LaneId, Point)> =
            road.lanes.iter().enumerate()
            .map(|tuple| {
                let (id, lane) = tuple;
                let id = LaneId { id };
                let pos = lane_to_streetlight_position(
                    road, lane, config.streetlight_distance);
                (id, pos)
            })
            .collect();

        Self {
            vertex_buffer,
            index_buffer,
            textures: vec![red, green, yellow],
            program,
            streetlights,
        }
    }

    pub fn render<T>(
        &self, target: &mut T, road: &Road,
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

        for &(lane, pos) in self.streetlights.iter() {
            let world = na::Matrix4::identity()
                .append_translation(&na::Vector3::new(
                    pos.x, pos.y, 0.0));

            let matrix = *view_proj * world;
            let matrix_ref: &[[f32; 4]; 4] = matrix.as_ref();

            let color_index = match road.get_street_light_color(lane) {
                Color::Red => RED_INDEX,
                Color::Green => GREEN_INDEX,
                Color::Yellow => YELLOW_INDEX,
            };

            let uniform = uniform! {
                matrix: *matrix_ref,
                tex: &self.textures[color_index],
            };

            target.draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &self.program,
                &uniform, 
                &params).unwrap();
        }
    }
}
