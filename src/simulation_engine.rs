use sdl2;
use sdl2::event::Event;

use time;
use time::Duration;

use crate::brushes;
use crate::counter::Counter;
use crate::material::Material;
use crate::material_map::MaterialMap;
use crate::window;

pub struct SimulationEngine {
    buffer_width: usize,
    buffer_height: usize,
    time_at_last_update: time::Instant,
    time_at_last_render: time::Instant,
    generation_counter: Counter,
    map: Box<MaterialMap>,
    mouse_button_down: bool,
    selected_material: Material,
    pixel_buffer: [u8; window::SCREEN_WIDTH * window::SCREEN_HEIGHT * 3],
    // Consider moving this into a different struct
    elapsed: Duration,
    frame_counter: i32,
    update_counter: i32,
    updating: bool,
    generator: bool,
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
            selected_material: Material::Sand,
            map: Box::new(MaterialMap::new(width, height)),
            pixel_buffer: [0; window::SCREEN_HEIGHT * window::SCREEN_WIDTH * 3],
            elapsed: Duration::seconds(0),
            frame_counter: 0,
            update_counter: 0,
            updating: true,
            generator: false,
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match *event {
            Event::KeyUp { keycode, .. } => match keycode.unwrap() {
                // https://docs.rs/sdl2/latest/sdl2/keyboard/enum.Keycode.html
                sdl2::keyboard::Keycode::S => {
                    self.selected_material = Material::Sand;
                }
                sdl2::keyboard::Keycode::E => {
                    self.selected_material = Material::Explosive;
                }
                sdl2::keyboard::Keycode::F => {
                    self.selected_material = Material::Fire {
                        duration: 30,
                        pressure: 0,
                    };
                }
                sdl2::keyboard::Keycode::P => {
                    self.selected_material = Material::Pressure;
                }
                sdl2::keyboard::Keycode::W => {
                    self.selected_material = Material::Wood;
                }
                sdl2::keyboard::Keycode::C => {
                    self.selected_material = Material::Cardboard;
                }
                sdl2::keyboard::Keycode::Period => {
                    self.generator = !self.generator;
                }
                sdl2::keyboard::Keycode::Space => {
                    self.updating = !self.updating;
                }
                _ => {}
            },
            Event::MouseButtonDown { x, y, .. } => {
                self.mouse_button_down = true;
                println!("(Y, X) ({}, {})", y, x);
                for cord in
                    brushes::circle(5.0, y, x, self.buffer_height, self.buffer_width, 0.00001)
                {
                    self.add_selected_to_map(cord.0 as usize, cord.1 as usize);
                }
            }
            Event::MouseButtonUp { .. } => self.mouse_button_down = false,
            Event::MouseMotion { x, y, .. } => {
                if self.mouse_button_down {
                    for cord in
                        brushes::circle(5.0, y, x, self.buffer_height, self.buffer_width, 0.00001)
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
        self.map.add_material(y, x, mat);
    }

    pub fn update(&mut self, texture: &mut sdl2::render::Texture) {
        let previous_update = self.time_at_last_update;
        let time_elapsed = time::Instant::now() - previous_update;

        if self.updating {
            if time_elapsed >= time::Duration::milliseconds(10) {
                self.update_cell_positions(&time_elapsed);
                self.time_at_last_update = time::Instant::now();
                self.update_counter = self.update_counter + 1;
            }

            if self.generator && self.generation_counter.elapsed_gt(20) {
                for cord in
                    brushes::circle(10.0, 10, 600, self.buffer_height, self.buffer_width, 0.9)
                {
                    self.map.add_material(cord.0 as usize, cord.1 as usize, Material::Sand);
                }

                self.generation_counter.reset();
            }
        }

        self.update_texture(texture);

        self.frame_counter = self.frame_counter + 1;
        let last_render_time = self.time_at_last_render;
        self.time_at_last_render = time::Instant::now();
        let time_between_render = self.time_at_last_render - last_render_time;
        self.elapsed = self.elapsed + time_between_render;

        if self.elapsed > time::Duration::seconds(1) {
            println!(
                "FPS {} - Updates/Second {}",
                self.frame_counter, self.update_counter
            );
            self.frame_counter = 0;
            self.update_counter = 0;
            self.elapsed = Duration::seconds(0);
        }
    }

    fn update_texture(&mut self, texture: &mut sdl2::render::Texture) {
        self.pixel_buffer = [0; window::SCREEN_HEIGHT * window::SCREEN_WIDTH * 3];
        for y in 0..self.buffer_height {
            for x in 0..self.buffer_width {
                if let Some(cell) = self.map.contents_at_index(y, x) {
                    let offset = y * window::SCREEN_WIDTH * 3 + x * 3;
                    let rgb = cell.mat.rgb();
                    self.pixel_buffer[offset + 0] = rgb.red as u8;
                    self.pixel_buffer[offset + 1] = rgb.green as u8;
                    self.pixel_buffer[offset + 2] = rgb.blue as u8;
                }
            }
        }
        texture.update(None, &self.pixel_buffer, 2400).unwrap();
    }
}

pub trait UpdateCellPositions {
    fn update_cell_positions(&mut self, _elapsed: &time::Duration);
    fn gravity(&mut self);
    fn fire(&mut self);
    fn pressure(&mut self);
}

impl UpdateCellPositions for SimulationEngine {
    fn update_cell_positions(&mut self, _elapsed: &time::Duration) {
        self.gravity();
        self.fire();
        self.pressure();
        self.map.apply_forces();
    }

    fn gravity(&mut self) {
        for y in 0..self.buffer_height {
            for x in 0..self.buffer_width {
                self.map.add_force_at_index(y, x, -1, 0);
            }
        }
    }

    fn fire(&mut self) {
        for y in 0..self.buffer_height {
            for x in 0..self.buffer_width {
                if let Some(mat) = self.map.contents_at_index(y, x) {
                    match mat.mat {
                        Material::Fire { duration, pressure } => {
                            // Deteriorate the fire
                            if duration <= 0 && pressure > 0 {
                                self.map.add_material(
                                    y,
                                    x,
                                    Material::Pressure,
                                );
                            } else if duration > 0 {
                                self.map.add_material(
                                    y,
                                    x,
                                    Material::Fire {
                                        duration: duration - 1,
                                        pressure,
                                    },
                                );
                            } else {
                                self.map.remove_at_position(y, x);
                            }
                        }
                        _ => {
                            continue;
                        }
                    }
                } else {
                    continue;
                }
                // Catch everything else on fire
                for yi in (std::cmp::max(1, y) - 1)..(y + 2) {
                    for xi in (std::cmp::max(1, x) - 1)..(x + 2) {
                        if yi == y && xi == x {
                            continue; // Skip the pressure instance
                        }
                        if let Some(contents) = self.map.contents_at_index(yi, xi) {
                            match contents.mat {
                                Material::Explosive => {
                                    self.map.add_material(
                                        yi,
                                        xi,
                                        Material::Fire {
                                            duration: 15,
                                            pressure: 20,
                                        },
                                    );
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    fn pressure(&mut self) {
        // For each Pressure instance on the grid, apply outward forces
        let power: usize = 20;
        let min_touched_pct: f64 = 0.4;
        for y in 0..self.buffer_height {
            for x in 0..self.buffer_width {
                if let Some(mat) = self.map.contents_at_index(y, x) {
                    if mat.mat != Material::Pressure {
                        continue; // If not a Pressure tile, we don't care
                    }
                } else {
                    continue;
                }
                let mut num_touched: i64 = 0;
                let mut num_possible: i64 = 0;
                for yi in (std::cmp::max(power, y) - power)..(y + power) {
                    for xi in (std::cmp::max(power, x) - power)..(x + power) {
                        if yi == y && xi == x {
                            continue; // Skip the pressure instance
                        }
                        num_possible += 1;

                        // TODO: Maybe add higher force for closer objects
                        let force_y = if yi < y { 1 } else { -1 };
                        let force_x = if xi < x { -1 } else { 1 };

                        // Push objects outwards if they're within a certain distance
                        let distance = ((x as f64 - xi as f64).powf(2.0)
                            + (y as f64 - yi as f64).powf(2.0))
                        .sqrt();
                        if distance < power as f64 {
                            if let Some(mat) = self.map.contents_at_index(yi, xi) {
                                if mat.mat != Material::Pressure {
                                    self.map.add_force_at_index(yi, xi, force_y, force_x);
                                    num_touched += 1;
                                }
                            }
                        }
                    }
                }
                if num_touched as f64 / (num_possible as f64) < min_touched_pct {
                    self.map.remove_at_position(y, x);
                }
            }
        }
    }
}
