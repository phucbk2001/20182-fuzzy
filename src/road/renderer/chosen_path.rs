use super::*;

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

impl RoadRenderer {

    pub fn update_chosen_path(
        &mut self, display: &Display, path: &[LocationId]) 
    {
        if path.len() == 0 {
            self.chosen_index_buffer = 
                IndexBuffer::empty(
                    display,
                    glium::index::PrimitiveType::LinesList,
                    0
                ).unwrap();
        }
        else {
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

}
