use crate::bodies;
use crate::cell::Cell;
use crate::cell::MaterialRecord;
use crate::material::Material;
use crate::material::RGB;
use crate::window;

pub struct MaterialMap {
    map_width: usize,
    map_height: usize,
    max_index: usize,
    pub mat_map: [Cell; window::SCREEN_HEIGHT * window::SCREEN_WIDTH],
}

impl MaterialMap {
    pub fn new(width: usize, height: usize) -> MaterialMap {
        MaterialMap {
            map_width: width,
            map_height: height,
            max_index: (height - 1) * width + (width - 1),
            mat_map: MaterialMap::new_mat_map(),
        }
    }

    fn new_mat_map() -> [Cell; window::SCREEN_HEIGHT * window::SCREEN_WIDTH] {
        [Cell::default(); window::SCREEN_WIDTH * window::SCREEN_HEIGHT]
    }

    fn index(&self, y: usize, x: usize) -> usize {
        // Convert coordinate into the index in the MaterialMap array
        y * self.map_width + x
    }

    fn index_with_force(
        &self,
        orig_y: usize,
        orig_x: usize,
        force_y: i8,
        force_x: i8,
    ) -> usize {
        // Convert coordinate with forces into destination index
        let mut y = orig_y;
        let mut x = orig_x;
        if y > 0 && force_y > 0 {
            y -= 1;
        } else if force_y < 0 {
            y += 1;
        }
        if x < self.map_width && force_x > 0 {
            x += 1;
        } else if force_x < 0 {
            x -= 1;
        }
        self.index(y, x)
    }

    pub fn add_material(&mut self, y: usize, x: usize, material: Material) {
        let m = MaterialRecord {
            mat: material,
            force_y: 0i8,
            force_x: 0i8,
        };
        self.mat_map[self.index(y, x)].contents = Some(m);
    }

    pub fn add_force_at_index(&mut self, y: usize, x: usize, force_y: i8, force_x: i8) {
        let index = self.index(y, x);
        if index > self.max_index {
            return;
        }
        if let Some(i) = self.mat_map[index].contents.as_mut() {
            i.force_y = std::cmp::min(i.force_y as i16 + force_y as i16, i8::MAX as i16) as i8;
            i.force_x = std::cmp::min(i.force_x as i16 + force_x as i16, i8::MAX as i16) as i8;
        }
    }

    pub fn override_force_at_index(&mut self, y: usize, x: usize, force_y: i8, force_x: i8) {
        let index = self.index(y, x);
        if index > self.max_index {
            return;
        }
        if let Some(i) = self.mat_map[index].contents.as_mut() {
            i.force_y = force_y;
            i.force_x = force_x;
        }
    }

    pub fn something_at_index(&self, y: usize, x: usize) -> bool {
        let index = self.index(y, x);
        if index > self.max_index {
            return false;
        }
        self.mat_map[index].contents.is_some()
    }

    pub fn material_at_index(&self, y: usize, x: usize) -> Material {
        self.mat_map[self.index(y, x)].contents.unwrap().mat
    }

    pub fn contents_at_index(&self, y: usize, x: usize) -> Option<MaterialRecord> {
        let index = self.index(y, x);
        if index > self.max_index {
            return None;
        }
        self.mat_map[index].contents
    }

    pub fn rgb_at_index(&self, y: usize, x: usize) -> RGB {
        self.mat_map[self.index(y, x)].contents.unwrap().mat.rgb()
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
        self.mat_map[self.index(y, x)].contents = None;
    }

    pub fn apply_forces(&mut self) {
        let mut new_mat_map = MaterialMap::new_mat_map();

        // Given the current forces on each object, average them all then override each
        // pixel's force with the average. This way we can get bodies to move together.
        let bodies = bodies::find_bodies(&self, self.map_height, self.map_width);

        for body in bodies {
            // Determine the average forces on the body
            let mut total_force_y = 0 as i64;
            let mut total_force_x = 0 as i64;
            let mut num_pixels = 0 as i64;
            // Special Y axis tracking so we can hit ground
            let mut max_y = 0;
            for coord in &body {
                let contents = self.contents_at_index(coord.0, coord.1).unwrap();
                total_force_y += contents.force_y as i64;
                total_force_x += contents.force_x as i64;
                num_pixels += 1;
                max_y = std::cmp::max(max_y, coord.0);
            }

            // Override the forces
            let mut avg_force_y = total_force_y / num_pixels;
            let avg_force_x = total_force_x / num_pixels;
            for coord in &body {
                let mut contents = self.contents_at_index(coord.0, coord.1).unwrap();
                contents.force_y = 0;
                contents.force_x = 0;
                if avg_force_y < 0 && max_y == self.map_height - 1 {
                    avg_force_y = 0; // Prevent materials from falling below floor
                }
                let new_index = self.index_with_force(
                    coord.0,
                    coord.1,
                    avg_force_y as i8,
                    avg_force_x as i8,
                );
                if new_index > self.max_index {
                    continue; // Item fell outside the map
                }
                new_mat_map[new_index].contents = Some(contents);
            }
        }

        self.mat_map = new_mat_map;
    }
}
