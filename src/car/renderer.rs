use nalgebra as na;

use crate::bezier;
use crate::config::Config;
use crate::glhelper;
use crate::car::CarSystem;
use crate::car::CarType;
use crate::car::AddCar;

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
    mark_texture: Texture2d,
    mark_vertex_buffer: VertexBuffer,
    mark_index_buffer: IndexBuffer,
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

        let texture_car_normal = glhelper::load_texture(
            "assets/car1.png",
            image::ImageFormat::PNG,
            display
        );

        let texture_car_slow = glhelper::load_texture(
            "assets/car2.png",
            image::ImageFormat::PNG,
            display
        );
        
        let mark_texture = glhelper::load_texture(
            "assets/mark.png",
            image::ImageFormat::PNG,
            display
        );

        let mark_width = config.location_mark_width;
        let mark_height = config.location_mark_height;

        let mark_vertices = vec![
            Vertex { position: [-mark_width / 2.0, 0.0], tex_coords: [0.0, 0.0] },
            Vertex { position: [mark_width / 2.0, 0.0], tex_coords: [1.0, 0.0] },
            Vertex { position: [mark_width / 2.0, mark_height], tex_coords: [1.0, 1.0] },
            Vertex { position: [-mark_width / 2.0, mark_height], tex_coords: [0.0, 1.0] },
        ];

        let mark_vertex_buffer = VertexBuffer::new(
            display, 
            &mark_vertices,
        ).unwrap();

        let mark_indices: Vec<u16> = vec![0, 1, 2, 2, 3, 0];

        let mark_index_buffer = IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &mark_indices,
        ).unwrap();

        Self {
            vertex_buffers: vec![vertex_buffer],
            index_buffers: vec![index_buffer],
            textures: vec![texture_car_normal, texture_car_slow],
            mark_texture,
            mark_vertex_buffer,
            mark_index_buffer,
            program,
        }
    }

    fn get_texture(&self, car_type: CarType) -> &Texture2d {
        match car_type {
            CarType::Normal(_) => &self.textures[0],
            CarType::Slow => &self.textures[1],
        }
    }

    pub fn render_mark<T>(
        &self, target: &mut T,
        view_proj: &na::Matrix4<f32>,
        params: &glium::draw_parameters::DrawParameters,
        p: bezier::Point)
        where T: Surface 
    {
        let world = na::Matrix4::identity()
            .append_translation(&na::Vector3::new(p.x, p.y, 0.0));
        let matrix = *view_proj * world;
        let matrix_ref: &[[f32; 4]; 4] = matrix.as_ref();

        let uniform = uniform! {
            matrix: *matrix_ref,
            tex: &self.mark_texture,
        };

        target.draw(
            &self.mark_vertex_buffer,
            &self.mark_index_buffer,
            &self.program,
            &uniform, 
            params).unwrap();
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
                    tex: self.get_texture(car.car_type),
                };

                target.draw(
                    &self.vertex_buffers[0],
                    &self.index_buffers[0],
                    &self.program,
                    &uniform, 
                    &params).unwrap();
            }
        }

        if let AddCar::AddedPoint(p) = car_system.add_car {
            self.render_mark(target, view_proj, &params, p);
        }

        if let Some(e) = car_system.chosen_car {
            if car_system.em.is_alive(e) {
                let p = car_system.cars.get(e).starting;
                self.render_mark(target, view_proj, &params, p);

                let p = car_system.cars.get(e).destination;
                self.render_mark(target, view_proj, &params, p);
            }
        }
    }
}
