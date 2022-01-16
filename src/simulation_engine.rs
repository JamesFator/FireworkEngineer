use sdl2;
use sdl2::event::Event;

use time;
use time::Duration;

use rand;

use crate::brushes;
use crate::counter::Counter;
use crate::material::Material;
use crate::material::State;
use crate::material_map::MaterialMap;
use crate::window;

pub struct SimulationEngine {
    buffer_width: usize,
    buffer_height: usize,
    time_at_last_update: time::Instant,
    time_at_last_render: time::Instant,
    generation_counter: Counter,
    map: MaterialMap,
    mouse_button_down: bool,
    selected_material: Material,
    pixel_buffer: [u8; window::SCREEN_WIDTH * window::SCREEN_HEIGHT * 3],
    // Consider moving this into a different struct
    cum_elapsed: Duration,
    frame_counter: i32,
    update_counter: i32,
}

impl SimulationEngine {
    pub fn new(width: usize, height: usize) -> SimulationEngine {
        SimulationEngine {
            buffer_width: width,
            buffer_height: height,
            time_at_last_update: time::Instant::now(),
            time_at_last_render: time::Instant::now(),
            generation_counter: Counter::new(),
            mouse_button_down: false,
            selected_material: Material::def_stone(),
            map: MaterialMap::new(width, height),
            pixel_buffer: [0; window::SCREEN_HEIGHT * window::SCREEN_WIDTH * 3],
            cum_elapsed: Duration::seconds(0),
            frame_counter: 0,
            update_counter: 0,
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match *event {
            Event::KeyUp { keycode, .. } if keycode.unwrap() == sdl2::keyboard::Keycode::K => {
                self.selected_material = Material::def_stone();
            }
            Event::KeyUp { keycode, .. } if keycode.unwrap() == sdl2::keyboard::Keycode::S => {
                self.selected_material = Material::def_sand();
            }
            Event::MouseButtonDown { .. } => {
                self.mouse_button_down = true;
            }
            Event::MouseButtonUp { .. } => self.mouse_button_down = false,
            Event::MouseMotion { x, y, .. } => {
                if self.mouse_button_down {
                    for cord in
                        brushes::circle(2.0, y, x, self.buffer_height, self.buffer_width, 0.00001)
                    {
                        self.add_selected_to_map(cord.0 as usize, cord.1 as usize);
                    }
                }
            }
            _ => {}
        }
    }

    fn add_selected_to_map(&mut self, y: usize, x: usize) {
        let mat = self.selected_material.clone();
        self.add_material_to_map(y, x, mat);
    }

    fn add_material_to_map(&mut self, y: usize, x: usize, material: Material) {
        if !self.map.something_at_index(y, x) {
            let offset = y * window::SCREEN_WIDTH * 3 + x * 3;
            let rgb = material.rgb();
            self.pixel_buffer[offset + 0] = rgb.red as u8;
            self.pixel_buffer[offset + 1] = rgb.green as u8;
            self.pixel_buffer[offset + 2] = rgb.blue as u8;
            self.map.add_material(y, x, material);
        }
    }

    pub fn update(&mut self, texture: &mut sdl2::render::Texture) {
        let previous_update = self.time_at_last_update;
        let time_elapsed = time::Instant::now() - previous_update;

        if time_elapsed >= time::Duration::milliseconds(10) {
            self.update_cell_positions(&time_elapsed);
            self.time_at_last_update = time::Instant::now();
            self.update_counter = self.update_counter + 1;
        }

        if self.generation_counter.elapsed_gt(20) {

            for cord in brushes::circle(10.0, 10, 600, self.buffer_height, self.buffer_width, 0.9) {
                self.add_material_to_map(cord.0 as usize, cord.1 as usize, Material::def_sand());
            }

            self.generation_counter.reset();
        }

        self.update_texture(texture);

        self.frame_counter = self.frame_counter + 1;
        let last_render_time = self.time_at_last_render;
        self.time_at_last_render = time::Instant::now();
        let time_between_render = self.time_at_last_render - last_render_time;
        self.cum_elapsed = self.cum_elapsed + time_between_render;

        if self.cum_elapsed > time::Duration::seconds(1) {
            println!("frames per second {}", self.frame_counter);
            println!("updates per second {}", self.update_counter);
            self.frame_counter = 0;
            self.update_counter = 0;
            self.cum_elapsed = Duration::seconds(0);
        }
    }

    fn update_texture(&mut self, texture: &mut sdl2::render::Texture) {
        texture.update(None, &self.pixel_buffer, 2400).unwrap();
    }
}

pub trait UpdateCellPositions {
    fn update_cell_positions(&mut self, _elapsed: &time::Duration);
    fn try_move_side_down(&mut self, y: usize, x: usize);
    fn handle_sand(&mut self, y: usize, x: usize);
    fn move_material(&mut self, yfrom: usize, xfrom: usize, yto: usize, yto: usize);
    fn remove_material(&mut self, y: usize, x: usize);
    fn bulk_move(&mut self, y: usize, x: usize);
    fn block_to_right(&self, y: usize, x: usize, count: usize) -> bool;
    fn block_to_left(&self, y: usize, x: usize, count: usize) -> bool;
    fn block_above(&self, y: usize, x: usize, count: usize) -> bool;
    fn update_loop_inner(&mut self, y: usize, x: usize);
}

impl UpdateCellPositions for SimulationEngine {
    fn update_cell_positions(&mut self, _elapsed: &time::Duration) {
        self.map.reset_states();
        for y in 0..self.buffer_height {
            if rand::random::<bool>() {
                for x in 0..self.buffer_width {
                    self.update_loop_inner(y, x);
                }
            } else {
                for x in (0..self.buffer_width).rev() {
                    self.update_loop_inner(y, x);
                }
            }
        }
    }

    fn update_loop_inner(&mut self, y: usize, x: usize) {
        if self.map.something_at_index(y, x) && self.map.state_at_index(y, x) == State::Free {
            match self.map.material_at_index(y, x) {
                Material::Sand => self.handle_sand(y, x),
                Material::Stone => (),
            }
        }
    }

    fn move_material(&mut self, yfrom: usize, xfrom: usize, yto: usize, xto: usize) {
        self.map.move_material(yfrom, xfrom, yto, xto);
        let offset_from = yfrom * window::SCREEN_WIDTH * 3 + (xfrom * 3);
        let offset_to = yto * window::SCREEN_WIDTH * 3 + (xto * 3);

        let tmp_red = self.pixel_buffer[offset_to + 0];
        let tmp_green = self.pixel_buffer[offset_to + 1];
        let tmp_blue = self.pixel_buffer[offset_to + 2];

        self.pixel_buffer[offset_to + 0] = self.pixel_buffer[offset_from + 0];
        self.pixel_buffer[offset_to + 1] = self.pixel_buffer[offset_from + 1];
        self.pixel_buffer[offset_to + 2] = self.pixel_buffer[offset_from + 2];

        self.pixel_buffer[offset_from + 0] = tmp_red;
        self.pixel_buffer[offset_from + 1] = tmp_green;
        self.pixel_buffer[offset_from + 2] = tmp_blue;
    }

    fn remove_material(&mut self, y: usize, x: usize) {
        self.map.remove_at_position(y, x);
        let offset = y * window::SCREEN_WIDTH * 3 + x * 3;

        self.pixel_buffer[offset + 0] = 0;
        self.pixel_buffer[offset + 1] = 0;
        self.pixel_buffer[offset + 2] = 0;
    }

    fn handle_sand(&mut self, y: usize, x: usize) {
        if y < (self.buffer_height - 1)
            && self.map.something_at_index(y + 1, x)
            && x > 0
            && x < (self.buffer_width - 1)
        {
            self.try_move_side_down(y, x);
        } else if y < (self.buffer_height - 1) && !self.map.something_at_index(y + 1, x) {
            self.move_material(y, x, y + 1, x);
            self.map.change_state_at_index(y + 1, x, State::Set);
        } else {
            self.remove_material(y, x);
        }
    }

    fn try_move_side_down(&mut self, y: usize, x: usize) {
        if rand::random::<bool>() {
            if !self.map.something_at_index(y, x + 1) {
                self.move_material(y, x, y, x + 1);
                self.map.change_state_at_index(y, x + 1, State::Set);
            } else if !self.map.something_at_index(y + 1, x + 1) {
                self.move_material(y, x, y + 1, x + 1);
                self.map.change_state_at_index(y + 1, x + 1, State::Set);
            } else {
                self.map.change_state_at_index(y, x, State::Set);
            }
        } else {
            if !self.map.something_at_index(y, x - 1) {
                self.move_material(y, x, y, x - 1);
                self.map.change_state_at_index(y, x - 1, State::Set);
            } else if !self.map.something_at_index(y + 1, x - 1) {
                self.move_material(y, x, y + 1, x - 1);
                self.map.change_state_at_index(y + 1, x - 1, State::Set);
            } else {
                self.map.change_state_at_index(y, x, State::Set);
            }
        }
    }

    fn block_to_right(&self, y: usize, x: usize, count: usize) -> bool {
        let mut ret = true;
        for i in 0..count {
            ret = ret && self.map.something_at_index(y, x + i);
        }
        ret
    }

    fn block_to_left(&self, y: usize, x: usize, count: usize) -> bool {
        let mut ret = true;
        for i in 0..count {
            ret = ret && self.map.something_at_index(y, x - i);
        }
        ret
    }

    fn block_above(&self, y: usize, x: usize, count: usize) -> bool {
        let mut ret = true;
        for i in 0..count {
            ret = ret && self.map.something_at_index(y + i, x);
        }
        ret
    }

    fn bulk_move(&mut self, y: usize, x: usize) {
        if y > 4 && y < (self.buffer_height - 4) {
            if self.block_above(y, x, 4) {
                if self.block_to_right(y, x, 4) {
                    if !self.map.something_at_index(y, x - 3) {
                        self.move_material(y, x, y, x - 3);
                        self.map.change_state_at_index(y, x - 3, State::Set);
                    } else {
                        self.map.change_state_at_index(y, x, State::Set);
                    }
                } else if self.block_to_left(y, x, 4) {
                    if !self.map.something_at_index(y, x + 3) {
                        self.move_material(y, x, y, x + 3);
                        self.map.change_state_at_index(y, x + 3, State::Set);
                    } else {
                        self.map.change_state_at_index(y, x, State::Set);
                    }
                }
            }
        } else {
            self.map.change_state_at_index(y, x, State::Set);
        }
    }
}
