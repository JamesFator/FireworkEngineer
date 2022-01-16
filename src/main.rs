extern crate rand;
extern crate sdl2;
extern crate time;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

use firework_engineer::simulation_engine::SimulationEngine;
use firework_engineer::window;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(
            "prowst",
            window::SCREEN_WIDTH as u32,
            window::SCREEN_HEIGHT as u32,
        )
        .position_centered()
        .opengl()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGB24,
            window::SCREEN_WIDTH as u32,
            window::SCREEN_HEIGHT as u32,
        )
        .map_err(|e| e.to_string())
        .unwrap();
    let mut simulation_engine = SimulationEngine::new(
        window::SCREEN_WIDTH as usize,
        window::SCREEN_HEIGHT as usize,
    );
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => simulation_engine.handle_event(&event),
            }
        }

        simulation_engine.update(&mut texture);

        canvas.clear();
        canvas
            .copy(
                &texture,
                None,
                Some(Rect::new(
                    0,
                    0,
                    window::SCREEN_WIDTH as u32,
                    window::SCREEN_HEIGHT as u32,
                )),
            )
            .unwrap();
        canvas.present();
    }
}
