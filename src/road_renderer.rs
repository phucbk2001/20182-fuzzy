use glium::implement_vertex;
use glium::{
    Program, Display, 
    VertexBuffer, IndexBuffer,
    Surface,
};
use glium::uniform;

use nalgebra as na;

use crate::bezier;
use crate::road;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

#[allow(dead_code)]
pub fn pos(x: f32, y: f32) -> Vertex {
    Vertex {
        position: [x, y],
    }
}

#[allow(dead_code)]
pub struct RoadRenderer {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,

    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    program: Program,
    pub road_color: [f32; 3],
}

const BEZIER_VCOUNT: u32 = 16;

const VERTEX_SHADER_SRC: &'static str = r#"
    #version 140

    in vec2 position;

    uniform mat4 matrix;

    void main() {
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER_SRC: &'static str = r#"
    #version 140
    out vec4 color;

    uniform vec3 road_color;

    void main() {
        color = vec4(road_color, 1.0);
    }
"#;

impl RoadRenderer {
    #[allow(dead_code)]
    pub fn new(display: &Display) -> Self {
        let program = glium::Program::from_source(
            display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();
        Self {
            vertices: vec![],
            indices: vec![],
            vertex_buffer: VertexBuffer::empty(display, 0).unwrap(),
            index_buffer: IndexBuffer::empty(
                display,
                glium::index::PrimitiveType::TrianglesList,
                0
            ).unwrap(),
            program: program,
            road_color: [0.4, 0.4, 0.7],
        }
    }

    fn update(&mut self, display: &Display) {
        self.vertex_buffer = VertexBuffer::new(
            display, 
            &self.vertices
        ).unwrap();
        self.index_buffer = IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &self.indices
        ).unwrap();
    }

    #[allow(dead_code)]
    pub fn render<T>(&self, target: &mut T, matrix: &na::Matrix4<f32>) 
        where T: Surface
    {
        let matrix_ref: &[[f32; 4]; 4] = matrix.as_ref();
        let uniform = uniform! {
            matrix: *matrix_ref,
            road_color: self.road_color,
        };

        target.draw(
            &self.vertex_buffer,
            &self.index_buffer,
            &self.program,
            &uniform, 
            &Default::default()).unwrap();
    }

    fn add_vertex(&mut self, p: bezier::Point) -> u16 {
        let index = self.vertices.len();
        let bezier::Point { x, y } = p;
        self.vertices.push(Vertex { position: [x, y] });
        index as u16
    }

    fn update_from_beziers(
        &mut self, 
        road: &road::Road,
        left: &Vec<road::DirectedBezier>,
        right: &Vec<road::DirectedBezier>)
    {
        let bezier_count = left.len();

        assert!(bezier_count > 0, "Len must not be zero");
        assert!(
            bezier_count == right.len(), 
            "Left and Right must be the same number of Beziers");

        let b1 = road.get_bezier(left[0]);
        let b2 = road.get_bezier(right[0]);

        let mut index1_prev = self.add_vertex(b1.pos(0.0));
        let mut index2_prev = self.add_vertex(b2.pos(0.0));

        for i in 0..bezier_count {
            let b1 = road.get_bezier(left[i]);
            let b2 = road.get_bezier(right[i]);
                
            for k in 0..BEZIER_VCOUNT {
                let v: f32 = (k + 1) as f32 / BEZIER_VCOUNT as f32;

                let a = b1.pos(v);
                let b = b2.pos(v);

                let i1 = self.add_vertex(a);
                let i2 = self.add_vertex(b);

                self.indices.extend_from_slice(
                    &[index1_prev, index2_prev, i1, i1, index2_prev, i2]);

                index1_prev = i1;
                index2_prev = i2;
            }
        }
    }

    #[allow(dead_code)]
    pub fn update_from(
        &mut self, display: &Display, 
        road: &road::Road) 
    {
        for lane in &road.lanes {
            self.update_from_beziers(road, &lane.left, &lane.right);
        }

        for cross_section in &road.cross_sections {
            self.update_from_beziers(
                road, &cross_section.left, &cross_section.right);
        }

        self.update(display);
    }
}
