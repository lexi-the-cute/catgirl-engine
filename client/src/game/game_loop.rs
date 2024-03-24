use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

use crate::render::fps::FPS;
use crate::window::window_state::WindowState;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop, EventLoopBuilder, EventLoopWindowTarget};

#[cfg(target_os = "android")]
use winit::platform::android::EventLoopBuilderExtAndroid; // Necessary for with_android_app

#[cfg(target_family = "wasm")]
use winit::platform::web::EventLoopExtWebSys;

/// Reference to FPS Tracker
static FPS_INSTANCE: OnceLock<Arc<Mutex<FPS>>> = OnceLock::new();

// http://gameprogrammingpatterns.com/game-loop.html
// https://zdgeier.com/wgpuintro.html
// https://sotrh.github.io/learn-wgpu/beginner/tutorial5-textures/#loading-an-image-from-a-file
/// Client game loop
///
/// # Errors
///
/// The event loop may not be created
///
/// # Panics
///
/// The event loop may not be created
// TODO (BIND): Implement `extern "C"`
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn client_game_loop() -> Result<(), String> {
    // Create the main loop
    debug!("Creating event loop...");
    #[cfg(not(target_os = "android"))]
    let event_loop: EventLoop<()> = EventLoopBuilder::new()
        .build()
        .expect("Could not create an event loop!");

    #[cfg(target_os = "android")]
    let event_loop: EventLoop<()> = EventLoopBuilder::new()
        .with_android_app(crate::game::ANDROID_APP.get().unwrap().to_owned())
        .build()
        .expect("Could not create an event loop!");

    /// Holds the window state in a way that's compatible with async
    #[allow(clippy::items_after_statements)]
    static WINDOW_STATE: Mutex<Option<WindowState>> = Mutex::new(None);
    let mut frame_tracker: MutexGuard<'_, FPS> = get_fps_instance().lock().unwrap();
    debug!("Starting event loop...");
    let event_loop_closure = move |event: Event<()>, window_target: &EventLoopWindowTarget<()>| {
        /* Update Order
         *
         * processInput() - Handles user input
         * update() - Handle game physics
         * render() - Handle graphics
         */

        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        // The control flow doesn't control the frame rate
        // window_target.set_control_flow(winit::event_loop::ControlFlow::Poll);

        // TODO: Determine if this should be selected depending on menus and pause state
        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        window_target.set_control_flow(winit::event_loop::ControlFlow::Wait);

        // Redraws the screen
        if WINDOW_STATE.lock().unwrap().is_some()
            && should_request_redraw(
                WINDOW_STATE.lock().unwrap().as_ref().unwrap(),
                &frame_tracker,
            )
        {
            WINDOW_STATE
                .lock()
                .unwrap()
                .as_ref()
                .unwrap()
                .window
                .request_redraw();
        } else {
            frame_tracker.reset_frame_count();
        }

        match event {
            // The close button was pressed
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                crate::window::events::close_requested(window_target);
            }

            Event::Resumed => {
                if WINDOW_STATE.lock().unwrap().is_some() {
                    crate::window::events::resumed_window(
                        WINDOW_STATE.lock().unwrap().as_mut().unwrap(),
                    );
                } else {
                    *WINDOW_STATE.lock().unwrap() =
                        Some(crate::window::events::create_window(window_target));

                    #[cfg(not(target_family = "wasm"))]
                    futures::executor::block_on(
                        WINDOW_STATE
                            .lock()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .initialize_graphics(),
                    );

                    #[cfg(target_family = "wasm")]
                    {
                        let await_graphics = async {
                            WINDOW_STATE
                                .lock()
                                .unwrap()
                                .as_mut()
                                .unwrap()
                                .initialize_graphics()
                                .await;
                        };

                        wasm_bindgen_futures::spawn_local(await_graphics);
                    }
                }
            }

            Event::Suspended => {
                crate::window::events::suspended_window();
            }

            // Keyboard keys were pressed
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { event, .. },
                ..
            } => {
                crate::window::events::pressed_key(event, window_target);
            }

            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                crate::window::events::clicked_mouse(state, button, window_target);
            }

            Event::WindowEvent {
                event: WindowEvent::Touch(touch),
                ..
            } => {
                crate::window::events::touched_screen(touch);
            }

            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Technically, this can fail if another window was resized before this one was created
                if WINDOW_STATE.lock().unwrap().is_some() {
                    crate::window::events::resized_window(
                        WINDOW_STATE.lock().unwrap().as_ref().unwrap(),
                        size,
                    );
                }
            }

            // Can be used to pause the game
            Event::WindowEvent {
                event: WindowEvent::Focused(focused),
                ..
            } => {
                crate::window::events::changed_focus(
                    WINDOW_STATE.lock().unwrap().as_mut().unwrap(),
                    focused,
                );
            }

            // Called every time the engine needs to refresh a frame
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                if WINDOW_STATE.lock().unwrap().is_some() {
                    let mutex_guard: std::sync::MutexGuard<'_, Option<WindowState<'_>>> =
                        WINDOW_STATE.lock().unwrap();
                    let window_state: &WindowState<'_> = mutex_guard.as_ref().unwrap();

                    // Graphics processing
                    crate::window::events::requested_redraw(window_state);

                    // Calculate FPS
                    if frame_tracker.one_second_passed() {
                        window_state.window.set_title(
                            format!(
                                "Catgirl Engine - {} FPS",
                                frame_tracker.get_average_counted_frames()
                            )
                            .as_str(),
                        );
                        frame_tracker.reset_frame_count();
                    } else {
                        frame_tracker.count_frame();
                    }
                }
            }

            // Last event to ever be executed on shutdown
            Event::LoopExiting => {
                crate::window::events::exiting_loop();
            }

            // New events are incoming
            Event::NewEvents(_) => {
                crate::window::events::new_events();
            }

            // About to wait for new events to arrive
            Event::AboutToWait => {
                crate::window::events::about_to_wait_event();
            }

            // Last event to ever be executed on shutdown
            Event::MemoryWarning => {
                crate::window::events::low_memory_warning();
            }

            // All unnamed events
            _ => {
                crate::window::events::unhandled_event(event);
            }
        }
    };

    #[cfg(not(target_family = "wasm"))]
    let _: Result<(), winit::error::EventLoopError> = event_loop.run(event_loop_closure);

    #[cfg(target_family = "wasm")]
    event_loop.spawn(event_loop_closure);

    Ok(())
}

/// Get or create the global instance of the FPS tracker
pub fn get_fps_instance() -> &'static Arc<Mutex<FPS>> {
    FPS_INSTANCE.get_or_init(|| Arc::new(Mutex::new(FPS::new())))
}

/// Determine if should request redraw
pub fn should_request_redraw(
    window_state: &WindowState,
    frame_tracker: &MutexGuard<'_, FPS>,
) -> bool {
    let less_than_cap: bool = frame_tracker.is_less_than_cap();
    let window_focused: bool = window_state.focused;
    let pause_when_unfocused: bool = utils::args::get_args().pause_when_unfocused;

    if pause_when_unfocused {
        less_than_cap && window_focused
    } else {
        less_than_cap
    }
}
