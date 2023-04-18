use std::thread;
use std::time::Duration;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{VideoSubsystem, Sdl, EventPump};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;

// This thread handles both rendering and input (aka the client)
pub fn start() -> Result<(), String> {
    let sdl_context: Sdl = sdl2::init()?;
    let video_subsystem: VideoSubsystem = sdl_context.video()?;

    let window: Window = video_subsystem.window("Alexis' Game Engine", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().build()
        .expect("could not make a canvas");

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump: EventPump = sdl_context.event_pump()?;
    let mut i: u8 = 0;
    'running: loop {
        // Handle Events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        
        // Update Screen
        i = (i + 1) % 255;
        render(&mut canvas, Color::RGB(i, 64, 255 - i));

        // Slow Down Rendering (60 FPS)
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

// Unused, May Move Render/Event Loop Here
fn render(canvas: &mut Canvas<Window>, color: Color) {
    canvas.set_draw_color(color);
    canvas.clear();
    canvas.present();
}