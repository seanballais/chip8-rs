extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    const CHIP8_WINDOW_WIDTH: u32 = 64;
    const CHIP8_WINDOW_HEIGHT: u32 = CHIP8_WINDOW_WIDTH / 2;
    const CHIP8_PIXEL_SCALE: u32 = 10;
    const WINDOW_WIDTH: u32 = CHIP8_WINDOW_WIDTH * CHIP8_PIXEL_SCALE;
    const WINDOW_HEIGHT: u32 = CHIP8_WINDOW_HEIGHT * CHIP8_PIXEL_SCALE;

    let window = video_subsystem
        .window("chip8-rs", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    'emulator_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'emulator_loop,
                _ => {}
            }
        }

        canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
