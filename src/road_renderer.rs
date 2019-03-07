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

use road::LocationId;

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

struct ChosenLaneCrossSection {
    lanes: Vec<(LocationId, LocationId)>,
    cross_sections: Vec<(LocationId, LocationId, LocationId)>,
}

impl ChosenLaneCrossSection {
    fn from(chosen_path: &[LocationId]) -> Self {
        let mut lanes: Vec<(LocationId, LocationId)> = Vec::new();
        let mut cross_sections: 
            Vec<(LocationId, LocationId, LocationId)> = Vec::new();

        let mut it = chosen_path.iter();
        let mut prev_location = *it.next().unwrap();
        for location in it {
            lanes.push((prev_location, *location));
            lanes.push((*location, prev_location));
            prev_location = *location;
        }

        let mut it = chosen_path.iter();
        let mut prev_prev_location = *it.next().unwrap();
        let mut prev_location: LocationId;
        if let Some(location) = it.next() {
            prev_location = *location;
            for location in it {
                cross_sections.push(
                    (prev_prev_location, prev_location, *location));
                cross_sections.push(
                    (*location, prev_location, prev_prev_location));
                prev_prev_location = prev_location;
                prev_location = *location;
            }
        }

        Self {
            lanes: lanes,
            cross_sections: cross_sections,
        }
    }

    fn contains_lane(&self, lane: (LocationId, LocationId)) -> bool {
        let (from, to) = lane;
        let result = self.lanes.iter().find(|(find_from, find_to)| {
            *find_from == from && *find_to == to 
        });
        match result {
            Some(_) => true,
            None => false,
        }
    }

    fn contains_cross_section(
        &self, 
        cross_section: (LocationId, LocationId, LocationId)
        ) -> bool 
    {
        let (from, across, to) = cross_section;
        let result = self.cross_sections.iter().find(
            |(find_from, find_across, find_to)| 
            {
                *find_from == from &&
                    *find_across == across &&
                    *find_to == to
            });
        match result {
            Some(_) => true,
            None => false,
        }
    }
}

struct LaneIndex {
    from: LocationId,
    to: LocationId,
    right_border_indices: Vec<u16>,
}

struct CrossSectionIndex {
    from: LocationId,
    across: LocationId,
    to: LocationId,
    right_border_indices: Vec<u16>,
}

#[allow(dead_code)]
pub struct RoadRenderer {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub border_indices: Vec<u16>,

    lane_indices: Vec<LaneIndex>,
    cross_section_indices: Vec<CrossSectionIndex>,

    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    border_index_buffer: IndexBuffer<u16>,
    chosen_index_buffer: IndexBuffer<u16>,

    program: Program,
    pub road_color: [f32; 3],
    pub border_color: [f32; 3],
    pub chosen_color: [f32; 3],
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

    uniform vec3 input_color;

    void main() {
        color = vec4(input_color, 1.0);
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
            border_indices: vec![],
            lane_indices: vec![],
            cross_section_indices: vec![],
            vertex_buffer: VertexBuffer::empty(display, 0).unwrap(),
            index_buffer: IndexBuffer::empty(
                display,
                glium::index::PrimitiveType::TrianglesList,
                0
            ).unwrap(),
            border_index_buffer: IndexBuffer::empty(
                display,
                glium::index::PrimitiveType::LinesList,
                0
            ).unwrap(),
            chosen_index_buffer: IndexBuffer::empty(
                display,
                glium::index::PrimitiveType::LinesList,
                0
            ).unwrap(),
            program: program,
            road_color: [0.4, 0.4, 0.4],
            border_color: [0.0, 0.5, 0.0],
            chosen_color: [1.0, 0.0, 0.0],
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
        self.border_index_buffer = IndexBuffer::new(
            display,
            glium::index::PrimitiveType::LinesList,
            &self.border_indices
        ).unwrap();
    }

    #[allow(dead_code)]
    pub fn render<T>(&self, target: &mut T, matrix: &na::Matrix4<f32>) 
        where T: Surface
    {
        let matrix_ref: &[[f32; 4]; 4] = matrix.as_ref();

        let uniform = uniform! {
            matrix: *matrix_ref,
            input_color: self.road_color,
        };
        target.draw(
            &self.vertex_buffer,
            &self.index_buffer,
            &self.program,
            &uniform, 
            &Default::default()).unwrap();

        let uniform = uniform! {
            matrix: *matrix_ref,
            input_color: self.border_color,
        };
        target.draw(
            &self.vertex_buffer,
            &self.border_index_buffer,
            &self.program,
            &uniform, 
            &Default::default()).unwrap();

        let uniform = uniform! {
            matrix: *matrix_ref,
            input_color: self.chosen_color,
        };
        target.draw(
            &self.vertex_buffer,
            &self.chosen_index_buffer,
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
        right: &Vec<road::DirectedBezier>,
        right_indices: &mut Vec<u16>)
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

                self.border_indices.extend_from_slice(
                    &[index1_prev, i1, index2_prev, i2]);

                right_indices.extend_from_slice(
                    &[index2_prev, i2]);

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
            let mut lane_index = LaneIndex {
                from: lane.from, 
                to: lane.to,
                right_border_indices: Vec::new(),
            };

            self.update_from_beziers(
                road, &lane.left, &lane.right, 
                &mut lane_index.right_border_indices);

            self.lane_indices.push(lane_index);
        }

        for cross_section in &road.cross_sections {
            let mut cross_section_index = CrossSectionIndex {
                from: cross_section.from,
                across: cross_section.across,
                to: cross_section.to,
                right_border_indices: Vec::new(),
            };

            self.update_from_beziers(
                road, &cross_section.left, &cross_section.right,
                &mut cross_section_index.right_border_indices);

            self.cross_section_indices.push(cross_section_index);
        }

        self.update(display);
    }

    #[allow(dead_code)]
    pub fn update_chosen_path(
        &mut self, display: &Display, path: &[LocationId]) 
    {
        let chosen = ChosenLaneCrossSection::from(path);
        let mut indices: Vec<u16> = vec![];

        for lane_index in self.lane_indices.iter() {
            let lane = (lane_index.from, lane_index.to);
            if chosen.contains_lane(lane) {
                indices.extend_from_slice(
                    &lane_index.right_border_indices);
            }
        }

        for cross_section_index in self.cross_section_indices.iter() {
            let cross_section = (
                cross_section_index.from, 
                cross_section_index.across, 
                cross_section_index.to);
            if chosen.contains_cross_section(cross_section) {
                indices.extend_from_slice(
                    &cross_section_index.right_border_indices);
            }
        }

        self.chosen_index_buffer = IndexBuffer::new(
            display,
            glium::index::PrimitiveType::LinesList,
            &indices
        ).unwrap();
    }
}
