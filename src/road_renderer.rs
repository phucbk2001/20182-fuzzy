use glium::implement_vertex;
use glium::{Display, VertexBuffer, IndexBuffer};

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

#[allow(dead_code)]
pub struct RoadRenderer {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
}

impl RoadRenderer {
    #[allow(dead_code)]
    pub fn new(display: &Display) -> Self {
        Self {
            vertices: vec![],
            indices: vec![],
            vertex_buffer: VertexBuffer::empty(display, 0).unwrap(),
            index_buffer: IndexBuffer::empty(
                display,
                glium::index::PrimitiveType::TrianglesList,
                0
            ).unwrap(),
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self, display: &Display) {
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
}
