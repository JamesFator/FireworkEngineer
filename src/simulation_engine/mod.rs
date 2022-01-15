use std::mem;
use std::vec;
use std::collections::HashMap;

use sdl2;
use sdl2::event::Event;

use time;
use time::Duration;

use material::Material;
use material::RGB;
use material::State;
use material_map::MaterialMap;
use brushes;
use window;
use counter::Counter;

pub mod trait_update_cell_positions;
use simulation_engine::trait_update_cell_positions::UpdateCellPositions;

pub struct SimulationEngine {
    buffer_width: usize,
    buffer_height: usize,
    time_at_last_update: time::SteadyTime,
    time_at_last_render: time::SteadyTime,
    generation_counter: Counter,
    map: MaterialMap,
    mouse_button_down: bool,
    selected_material: Material,
    pixel_buffer: [u8; window::SCREEN_WIDTH * window::SCREEN_HEIGHT * 3],
    // Consider moving this into a different struct
    cum_elapsed: Duration,
    frame_counter: i32,
    update_counter: i32,
    t_count: i32,
    p_count: i32,
}

impl SimulationEngine {
    pub fn new(width: usize, height: usize) -> SimulationEngine {
        SimulationEngine {
            buffer_width: width,
            buffer_height: height,
            time_at_last_update: time::SteadyTime::now(),
            time_at_last_render: time::SteadyTime::now(),
            generation_counter: Counter::new(),
            mouse_button_down: false,
            selected_material: Material::def_stone(),
            map: MaterialMap::new(width, height),
            pixel_buffer: [0; window::SCREEN_HEIGHT * window::SCREEN_WIDTH *3],
            cum_elapsed: Duration::seconds(0),
            frame_counter: 0,
            update_counter: 0,
            t_count: 0,
            p_count: 0
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match *event {
            Event::KeyUp {keycode, ..}
                if keycode.unwrap() == sdl2::keyboard::Keycode::K => {
                    self.selected_material = Material::def_stone();
            },
            Event::KeyUp {keycode, ..}
                if keycode.unwrap() == sdl2::keyboard::Keycode::S => {
                    self.selected_material = Material::def_sand();
            },
            Event::MouseButtonDown {..} => {
                self.mouse_button_down = true;
            },
            Event::MouseButtonUp {..} => {
                self.mouse_button_down = false
            },
            Event::MouseMotion {x, y, ..} => {
                if self.mouse_button_down {
                    for cord in brushes::circle( 2.0, y, x, self.buffer_height, self.buffer_width, 0.00001) {
                        self.add_selected_to_map(cord.0 as usize, cord.1 as usize);
                    }
                }
            },
            _ => {}
        }
    }

    fn add_selected_to_map(&mut self, y: usize, x: usize) {
        let mat = self.selected_material.clone();
        self.add_material_to_map(y, x, mat);
    }

    fn add_material_to_map(&mut self, y: usize, x: usize, material: Material ) {
        if !self.map.something_at_index(y, x) {
            let offset = y * window::SCREEN_WIDTH*3 + x * 3;
            let rgb = material.rgb();
            self.pixel_buffer[offset + 0] = rgb.red as u8;
            self.pixel_buffer[offset + 1] = rgb.green as u8;
            self.pixel_buffer[offset + 2] = rgb.blue as u8;
            self.map.add_material(y, x, material);
        }
    }

    pub fn update(&mut self, texture: &mut sdl2::render::Texture) {
        let previous_update = self.time_at_last_update;
        let time_elapsed = time::SteadyTime::now() - previous_update;

        if time_elapsed >= time::Duration::milliseconds(10) {
            self.update_cell_positions(&time_elapsed);
            self.time_at_last_update = time::SteadyTime::now();
            self.update_counter = self.update_counter + 1;
        }

        if self.generation_counter.elapsed_gt(20) {
            for cord in brushes::circle( 10.0, 10, 200, self.buffer_height, self.buffer_width, 0.9) {
                self.add_material_to_map(cord.0 as usize, cord.1 as usize, Material::def_water());
            }

            for cord in brushes::circle( 10.0, 10, 600, self.buffer_height, self.buffer_width, 0.9) {
                self.add_material_to_map(cord.0 as usize, cord.1 as usize, Material::def_sand());
            }

            self.generation_counter.reset();
        }

        self.update_texture(texture);

        self.frame_counter = self.frame_counter + 1;
        let last_render_time = self.time_at_last_render;
        self.time_at_last_render = time::SteadyTime::now();
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
