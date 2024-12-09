use std::sync::{Mutex, OnceLock};

use crate::window::window_state::WindowState;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy};

#[cfg(target_os = "android")]
use winit::platform::android::EventLoopBuilderExtAndroid; // Necessary for with_android_app

#[cfg(target_family = "wasm")]
use winit::platform::web::EventLoopExtWebSys;

/// Allows sending custom events to the event loop from the outside
static EVENT_LOOP_PROXY: OnceLock<EventLoopProxy<()>> = OnceLock::new();

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
pub fn client_game_loop() -> Result<(), String> {
    // Create the main loop
    debug!("Creating event loop...");
    #[cfg(not(target_os = "android"))]
    let event_loop: EventLoop<()> = EventLoop::builder()
        .build()
        .expect("Could not create an event loop!");

    #[cfg(target_os = "android")]
    let event_loop: EventLoop<()> = EventLoop::builder()
        .with_android_app(crate::game::ANDROID_APP.get().unwrap().to_owned())
        .build()
        .expect("Could not create an event loop!");

    // This'll be useful for triggering the event loop from the outside when in wait mode
    let _ = EVENT_LOOP_PROXY.set(event_loop.create_proxy());

    /// Holds the window state in a way that's compatible with async
    #[allow(clippy::items_after_statements)]
    static WINDOW_STATE: Mutex<Option<WindowState>> = Mutex::new(None);
    debug!("Starting event loop...");
    let event_loop_closure = move |event: Event<()>, window_target: &ActiveEventLoop| {
        /* Update Order
         *
         * processInput() - Handles user input
         * update() - Handle game physics
         * render() - Handle graphics
         */

        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        // window_target.set_control_flow(winit::event_loop::ControlFlow::Poll);
        //
        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        //
        // This will use [`winit::event_loop::ControlFlow::Wait`] when on menus and pause
        //   and will use [`winit::event_loop::ControlFlow::Poll`] when in game
        window_target.set_control_flow(winit::event_loop::ControlFlow::Wait);

        // Starts exit process when exit bool is set
        if utils::exit::is_exiting() {
            // window_target.set_control_flow(winit::event_loop::ControlFlow::Poll);
            window_target.exit();
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
                crate::window::events::changed_focus(focused);
            }

            // Called every time the engine needs to refresh a frame
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                if WINDOW_STATE.lock().unwrap().is_some() {
                    crate::window::events::requested_redraw(
                        WINDOW_STATE.lock().unwrap().as_ref().unwrap(),
                    );
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

            // New events are incoming
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

    // TODO: Update to run_app and spawn_app
    #[cfg(not(target_family = "wasm"))]
    let _: Result<(), winit::error::EventLoopError> = event_loop.run(event_loop_closure);

    #[cfg(target_family = "wasm")]
    event_loop.spawn(event_loop_closure);

    Ok(())
}

/// Advances game loop by one cycle
#[must_use]
pub fn advance_event_loop() -> bool {
    send_event(())
}

/// Send's User Event to event loop
#[must_use]
fn send_event(event: ()) -> bool {
    let event_loop_proxy_option: Option<&EventLoopProxy<()>> = EVENT_LOOP_PROXY.get();
    if let Some(event_loop_proxy) = event_loop_proxy_option {
        let result: Result<(), winit::event_loop::EventLoopClosed<()>> =
            event_loop_proxy.send_event(event);

        return result.is_ok();
    };

    false
}
