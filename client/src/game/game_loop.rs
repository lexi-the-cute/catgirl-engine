use crate::window::window_state::WindowState;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop, EventLoopBuilder};

#[cfg(target_os = "android")]
use winit::platform::android::EventLoopBuilderExtAndroid; // Necessary for with_android_app

// http://gameprogrammingpatterns.com/game-loop.html
// https://zdgeier.com/wgpuintro.html
// https://sotrh.github.io/learn-wgpu/beginner/tutorial5-textures/#loading-an-image-from-a-file
pub fn game_loop() -> Result<(), String> {
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

    let mut window_state: Option<WindowState> = None;
    debug!("Starting event loop...");
    let _ = event_loop.run(move |event, window_target| {
        /* Update Order
         *
         * processInput() - Handles user input
         * update() - Handle game physics
         * render() - Handle graphics
         */

        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        window_target.set_control_flow(winit::event_loop::ControlFlow::Poll);

        // TODO: Determine if this should be selected depending on menus and pause state
        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        // window_target.set_control_flow(winit::event_loop::ControlFlow::Wait);

        match event {
            // The close button was pressed
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                crate::window::events::close_requested(window_target);
            }

            Event::Resumed => {
                if let Some(window_state) = &mut window_state {
                    crate::window::events::resumed_window(window_state);
                } else {
                    window_state = Some(crate::window::events::create_window(window_target));
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
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Technically, this can fail if another window was resized before this one was created
                if let Some(window_state) = window_state.as_ref() {
                    crate::window::events::resized_window(window_state, size);
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
                if let Some(window_state) = window_state.as_ref() {
                    crate::window::events::requested_redraw(window_state);
                }
            }

            // Last event to ever be executed on shutdown
            Event::LoopExiting => {
                crate::window::events::exiting_loop();
            }

            // All unnamed events
            _ => {
                crate::window::events::unhandled_event(event);
            }
        }
    });

    Ok(())
}
