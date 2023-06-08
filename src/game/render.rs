#![cfg(feature="client")]

// https://sunjay.dev/learn-game-dev/refactor-player-struct.html

use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::Duration;

use sdl2::controller::{Button};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureCreator, Texture};

use sdl2::video::{Window, WindowContext, GLContext};
use sdl2::{VideoSubsystem, Sdl, EventPump, HapticSubsystem, GameControllerSubsystem};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::image::{self, LoadTexture, InitFlag, Sdl2ImageContext};
use sdl2::keyboard::Keycode;

use crate::game::entity::player::{Player, Direction};

/// cbindgen:ignore
#[allow(unused_doc_comments)]
extern "C" {
    // emscripten_set_main_loop_arg(em_arg_callback_func func, void *arg, int fps, int simulate_infinite_loop)
    #[allow(improper_ctypes)]  // Rust discord said linter is doing false positive here
    #[cfg(all(target_family="wasm", target_os="emscripten"))]
    fn emscripten_set_main_loop_arg(
        func: extern "C" fn(Box<&mut RenderLoopStruct>) -> bool,
        arg: Box<&mut RenderLoopStruct>,  // Either `&mut RLS` or `Box<&mut RLS>` should work according to Rust discord
        fps: std::os::raw::c_int,
        simulate_infinite_loop: std::os::raw::c_int
    );

    #[cfg(all(target_family="wasm", target_os="emscripten"))]
    fn create_webgl_context() -> i32;  // It's actually a EMSCRIPTEN_RESULT from <emscripten/html5.h>
}

#[repr(C)]
struct RenderLoopStruct<'a> {
    receive: Receiver<()>,  // Receive From Main Thread In Render Thread
    i: AtomicU8,  // Used for changing background color,

    // SDL Related Vars
    sdl_context: Sdl,
    game_controller_subsystem: GameControllerSubsystem,
    haptic_subsystem: HapticSubsystem,
    video_subsystem: VideoSubsystem,
    image_context: Sdl2ImageContext,
    canvas: Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    texture: Texture<'a>,
    event_pump: EventPump,

    // TODO: Move to Server (Physics) Thread
    player: Player,
    player_movement_speed: i32
}

// This thread handles both rendering and input (aka the client)
pub fn start(tx: Sender<()>, rx: Receiver<()>) {
    run(rx).map_err(|err: String| {
        error!("Render Crash: {:?}", err);
    }).ok();

    tx.send(()).ok();
}

fn run(rx: Receiver<()>) -> Result<(), String> {
    debug!("run(rx) on render called...");

    // TODO: Make Sure These Are Only Initialized Once
    let sdl_context: Sdl = sdl2::init()?;
    debug!("SDL2 Initialized...");

    let game_controller_subsystem: GameControllerSubsystem = sdl_context.game_controller()?;
    debug!("Game Controller Subsystem Initialized...");

    let haptic_subsystem: HapticSubsystem = sdl_context.haptic()?;
    debug!("Haptic Subsystem Initialized...");

    let video_subsystem: VideoSubsystem = sdl_context.video()?;
    debug!("Video Subsystem Initialized...");

    let image_context: Sdl2ImageContext = image::init(InitFlag::PNG | InitFlag::JPG)?;
    debug!("Image Context Initialized...");

    let window: Window = video_subsystem.window("Alexis' Game Engine", 800, 600)
                                        .position_centered()
                                        .resizable()
                                        .build()
                                        .expect("Could Not Make a Window");
    debug!("Window Created...");

    #[cfg(all(target_family="wasm", target_os="emscripten"))] {
        unsafe {
            let webgl_context_result: i32 = create_webgl_context();
            debug!("WebGL Context Created With Value '{:?}'...", webgl_context_result);
        }
    }

    let mut canvas: Canvas<Window> = window.into_canvas()
                                            .accelerated()
                                            .present_vsync()
                                            .build()
                                            .expect("Could Not Make a Canvas");
    debug!("Canvas Created...");

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    debug!("Canvas Setup...");

    let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();
    debug!("Texture Creator Retrieved...");

    // `assets/bardo.png` automatically translates to `/data/data/land.catgirl.engine/files/assets/bardo.png` on Android
    // Android returns an empty string for this particular asset error while the error works as intended on Linux
    let texture: Texture = texture_creator.load_texture("assets/bardo.png")?;
    debug!("Texture Loaded...");

    let event_pump: EventPump = sdl_context.event_pump()?;
    debug!("Event Pump Retrieved...");

    // TODO: Move To Server (Physics) Thread
    let player: Player = Player {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 26, 36),
        speed: 0,
        direction: Direction::None
        // inventory: Inventory {}
    };

    #[allow(unused_mut)]
    let mut loopstruct: RenderLoopStruct = RenderLoopStruct {
        receive: rx.into(), i: AtomicU8::new(0),
        sdl_context,
        game_controller_subsystem, haptic_subsystem,
        video_subsystem, image_context,
        canvas,
        texture_creator: &texture_creator, texture,
        event_pump,
        
        // TODO: Move To Server (Physics) Thread
        player,
        player_movement_speed: 20
    };

    debug!("Starting Render Loop...");
    #[cfg(all(target_family="wasm", target_os="emscripten"))]
    unsafe {
        emscripten_set_main_loop_arg(render_loop, Box::from(&mut loopstruct), 0, 1);
    }
    
    #[cfg(not(all(target_family="wasm", target_os="emscripten")))]
    loop {
        let exit_loop: bool = render_loop(Box::from(&mut loopstruct));
        if exit_loop {
            // Ending Loop
            break;
        }
    }
    debug!("Exiting Render Loop...");

    Ok(())
}

fn should_terminate_thread(loopstruct: &Box<&mut RenderLoopStruct>) -> bool {
    let rx: &Receiver<()> = &loopstruct.receive;

    match rx.try_recv() {
        Ok(_) => {
            return true;
        }
        Err(_) => {
            return false;
        }
    }
}

extern "C" fn render_loop(loopstruct: Box<&mut RenderLoopStruct>) -> bool {
    if should_terminate_thread(&loopstruct) {
        debug!("Terminating Render Thread...");
        return true;
    }

    // Handle Events
    for event in loopstruct.event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                debug!("Terminating Render Thread (Quit)...");
                return true;
            },
            Event::ControllerDeviceAdded { which: controller_id, .. } => {
                let game_controller_subsystem: &GameControllerSubsystem = &loopstruct.game_controller_subsystem;
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
                loopstruct.player.speed = loopstruct.player_movement_speed;
                loopstruct.player.direction = Direction::Left;
            },
            Event::ControllerButtonDown { button: Button::DPadRight, .. } |
            Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                loopstruct.player.speed = loopstruct.player_movement_speed;
                loopstruct.player.direction = Direction::Right;
            },
            Event::ControllerButtonDown { button: Button::DPadUp, .. } |
            Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                loopstruct.player.speed = loopstruct.player_movement_speed;
                loopstruct.player.direction = Direction::Up;
            },
            Event::ControllerButtonDown { button: Button::DPadDown, .. } |
            Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                loopstruct.player.speed = loopstruct.player_movement_speed;
                loopstruct.player.direction = Direction::Down;
            },
            Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
            Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
            Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
            Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                loopstruct.player.speed = 0;
                loopstruct.player.direction = Direction::None;
            },
            _ => {}
        }
    }
    
    // Update Player
    loopstruct.player.update();

    // Update Screen
    let mut i: u8 = loopstruct.i.load(Ordering::Relaxed);
    i = (i + 1) % 255;

    loopstruct.i.store(i, Ordering::Relaxed);

    render(&mut loopstruct.canvas, Color::RGB(i, 64, 255-i), &loopstruct.texture, &loopstruct.player).unwrap();

    // Slow Down Rendering (60 FPS)
    #[cfg(not(all(target_family="wasm", target_os="emscripten")))]
    thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

    return false;
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