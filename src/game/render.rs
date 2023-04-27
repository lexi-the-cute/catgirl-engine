// https://sunjay.dev/learn-game-dev/refactor-player-struct.html

use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::Duration;

use sdl2::controller::{Button};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureCreator, Texture};

use sdl2::video::{Window, WindowContext};
use sdl2::{VideoSubsystem, Sdl, EventPump, HapticSubsystem, GameControllerSubsystem};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::image::{self, LoadTexture, InitFlag, Sdl2ImageContext};
use sdl2::keyboard::Keycode;

use crate::game::entity::player::{Player, Direction};

// This thread handles both rendering and input (aka the client)
pub fn start(tx: Sender<()>, rx: Receiver<()>) {
    run(rx).map_err(|err: String| {
        error!("Render Crash: {:?}", err);
    }).ok();

    tx.send(()).ok();
}

fn run(rx: Receiver<()>) -> Result<(), String> {
    // TODO: Make Sure These Three Lines Are Only Initialized Once
    let sdl_context: Sdl = sdl2::init()?;
    let game_controller_subsystem: GameControllerSubsystem = sdl_context.game_controller()?;
    let _haptic_subsystem: HapticSubsystem = sdl_context.haptic()?;
    let video_subsystem: VideoSubsystem = sdl_context.video()?;
    let _image_context: Sdl2ImageContext = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window: Window = video_subsystem.window("Alexis' Game Engine", 800, 600)
        .position_centered()
        .build().expect("Could Not Make A Window");

    let mut canvas: Canvas<Window> = window.into_canvas()
        .build().expect("Could Not Make a Canvas");

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();

    // `assets/bardo.png` automatically translates to `/data/data/land.catgirl.engine/files/assets/bardo.png` on Android
    // Android returns an empty string for this particular asset error while the error works as intended on Linux
    let texture: Texture = texture_creator.load_texture("assets/bardo.png")?;

    // TODO: Move To Server Thread
    const PLAYER_MOVEMENT_SPEED: i32 = 20;
    let mut player: Player = Player {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 26, 36),
        speed: 0,
        direction: Direction::None
        // inventory: Inventory {}
    };

    let mut event_pump: EventPump = sdl_context.event_pump()?;
    let mut i: u8 = 0;

    'running: loop {
        match rx.try_recv() {
            Ok(_) => {
                debug!("Terminating Render Thread...");
                break 'running;
            }
            Err(_) => {
                // Not Implemented At The Moment
            }
        }

        // Handle Events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    debug!("Terminating Render Thread (Quit)...");
                    break 'running;
                },
                Event::ControllerDeviceAdded { which: controller_id, .. } => {
                    let controller_name: String = game_controller_subsystem.name_for_index(controller_id).unwrap();
                    debug!("Controller {} ({}) Added...", controller_id, controller_name);
                    
                    // TODO: Determine Why Controller Input/Output Error
                    game_controller_subsystem.is_game_controller(controller_id);
                    game_controller_subsystem.open(controller_id).unwrap();
                    // .set_rumble(1000, 1000, 1000).unwrap();
                },
                Event::ControllerDeviceRemoved { which: controller_id, .. } => {
                    debug!("Controller {} Removed...", controller_id);
                },
                Event::ControllerButtonDown { button: Button::DPadLeft, .. } |
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Left;
                },
                Event::ControllerButtonDown { button: Button::DPadRight, .. } |
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Right;
                },
                Event::ControllerButtonDown { button: Button::DPadUp, .. } |
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Up;
                },
                Event::ControllerButtonDown { button: Button::DPadDown, .. } |
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Down;
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    player.speed = 0;
                    player.direction = Direction::None;
                },
                _ => {}
            }
        }
        
        // Update Player
        player.update();

        // Update Screen
        i = (i + 1) % 255;
        render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, &player)?;

        // Slow Down Rendering (60 FPS)
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

// Unused, May Move Render/Event Loop Here
fn render(canvas: &mut Canvas<Window>, color: Color, texture: &Texture, player: &Player) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;
    let screen_position: Point = player.get_position() + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect: Rect = Rect::from_center(screen_position, player.get_sprite().width(), player.get_sprite().height());

    canvas.copy(texture, player.get_sprite(), screen_rect)?;
    canvas.present();

    Ok(())
}