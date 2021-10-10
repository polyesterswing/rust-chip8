mod chip8;
use chip8::Chip8;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

fn main() {
    let mut chip8 = Chip8::new();

    chip8.load_program();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Chip8 Emulator", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB332, 64, 32)
        .map_err(|e| e.to_string())
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        chip8.cycle();

        let mut display: [u8; 4096] = [0; 4096];

        for pixel in chip8.vmem.iter().enumerate() {
            display[pixel.0] = if *pixel.1 == true {255} else {0};
        }

        texture.update(
            None,
            &display[..],
            64,
        ).unwrap();

        canvas.clear();
        canvas.copy(&texture, None, Some(Rect::new(100, 100, 64, 32))).unwrap();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
    }

}
