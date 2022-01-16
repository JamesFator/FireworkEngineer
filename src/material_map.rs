use crate::cell::Cell;
use crate::cell::MaterialRecord;
use crate::material::Material;
use crate::material::State;
use crate::material::RGB;
use crate::window;

pub struct MaterialMap {
    map_width: usize,
    map_height: usize,
    pub mat_map: [Cell; window::SCREEN_HEIGHT * window::SCREEN_WIDTH],
}

impl MaterialMap {
    pub fn new(width: usize, height: usize) -> MaterialMap {
        MaterialMap {
            map_width: width,
            map_height: height,
            mat_map: [Cell::default(); window::SCREEN_WIDTH * window::SCREEN_HEIGHT],
        }
    }

    pub fn add_material(&mut self, y: usize, x: usize, material: Material) {
        let m = MaterialRecord {
            mat: material,
            state: State::Free,
            force_y: 0i8,
            force_x: 0i8,
        };
        self.mat_map[y * self.map_width + x].contents = Some(m);
    }

    pub fn change_state_at_index(&mut self, y: usize, x: usize, state: State) {
        if let Some(i) = self.mat_map[y * self.map_width + x].contents.as_mut() {
            i.state = state;
            if state == State::Free {
                i.force_y = 0;
                i.force_x = 0;
            }
        }
    }

    pub fn change_force_at_index(&mut self, y: usize, x: usize, force_y: i8, force_x: i8) {
        if let Some(i) = self.mat_map[y * self.map_width + x].contents.as_mut() {
            i.force_y += force_y;
            i.force_x += force_x;
        }
    }

    pub fn something_at_index(&self, y: usize, x: usize) -> bool {
        self.mat_map[y * self.map_width + x].contents.is_some()
    }

    pub fn material_at_index(&self, y: usize, x: usize) -> Material {
        self.mat_map[y * self.map_width + x].contents.unwrap().mat
    }

    pub fn contents_at_index(&self, y: usize, x: usize) -> Option<MaterialRecord> {
        self.mat_map[y * self.map_width + x].contents
    }

    pub fn state_at_index(&self, y: usize, x: usize) -> State {
        self.mat_map[y * self.map_width + x].contents.unwrap().state
    }

    pub fn rgb_at_index(&self, y: usize, x: usize) -> RGB {
        self.mat_map[y * self.map_width + x]
            .contents
            .unwrap()
            .mat
            .rgb()
    }

    pub fn move_material(&mut self, yfrom: usize, xfrom: usize, yto: usize, xto: usize) {
        {
            let moving = &self.mat_map[yfrom * self.map_width + xfrom]
                .contents
                .unwrap();
            // To should always be none
            self.mat_map[yto * self.map_width + xto].contents = Some(*moving);
        }
        self.mat_map[yfrom * self.map_width + xfrom].contents = None;
    }

    pub fn remove_at_position(&mut self, y: usize, x: usize) {
        self.mat_map[y * self.map_width + x].contents = None;
    }

    pub fn reset_states(&mut self) {
        for y in 0..self.map_height {
            for x in 0..self.map_width {
                if self.something_at_index(y, x) {
                    self.change_state_at_index(y, x, State::Free);
                }
            }
        }
    }
}
