use crate::window::window_state::WindowState;

use winit::dpi::PhysicalSize;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::{EventLoop, EventLoopBuilder};
use winit::keyboard::{self, NamedKey};
use winit::window::{Window, WindowBuilder};

use wgpu::{
    Adapter, CommandEncoder, Device, Queue, RenderPass, RenderPassDescriptor, Surface,
    SurfaceTexture, TextureView,
};

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
                debug!("The Close Button Was Pressed! Stopping...");
                window_target.exit();
            }

            Event::Resumed => {
                debug!("Resuming application...");

                // Create window in an Atomically Reference Counted object
                // This is to allow retaining a handle to the window after passing it to create_surface
                // https://github.com/gfx-rs/wgpu/discussions/5213
                // https://doc.rust-lang.org/std/sync/struct.Arc.html
                if window_state.is_none() {
                    debug!("Creating window...");
                    let window = WindowBuilder::new()
                        .with_title("Catgirl Engine")
                        .with_window_icon(Some(crate::get_icon()))
                        .build(window_target)
                        .expect("Could not create window!");

                    window_state = Some(WindowState::new(window));
                } else {
                    window_state.as_mut().unwrap().recreate_surface();
                }
            }

            Event::Suspended => {
                debug!("Suspending application...");
            }

            // Keyboard keys were pressed
            // TODO: Offload to separate function with key mapping config
            // processInput()
            // update() - Input gets passed to (internal) server, physics gets passed back
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: keyboard::Key::Named(NamedKey::Escape),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                debug!("The Escape Key Was Pressed! Stopping...");
                window_target.exit();
            }

            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                let window_state: &WindowState<'_> = window_state.as_ref().unwrap();
                let window: &Window = &window_state.window;
                let device: &Device = &window_state.device;
                let surface: &Surface = &window_state.surface;
                let adapter: &Adapter = &window_state.adapter;

                let size: PhysicalSize<u32> = window.inner_size();
                surface.configure(
                    device,
                    &surface
                        .get_default_config(adapter, size.width, size.height)
                        .expect("Could not get surface default config!"),
                );
            }

            // Can be used to pause the game
            Event::WindowEvent {
                event: WindowEvent::Focused(_focused),
                ..
            } => {
                // debug!("Focused: {_focused}");
            }

            // Called every time the engine needs to refresh a frame
            // TODO: Offload to separate function
            // render()
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // TODO: https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/#what-s-a-pipeline
                // Configure a surface for drawing on
                let window_state: &WindowState<'_> = window_state.as_ref().unwrap();
                let device: &Device = &window_state.device;
                let surface: &Surface = &window_state.surface;
                let queue: &Queue = &window_state.queue;

                // Get a texture to draw onto the surface
                // https://docs.rs/wgpu/latest/wgpu/struct.SurfaceTexture.html
                // https://stackoverflow.com/a/4262634
                // This segfaults when resizing window if no render commands are executed
                let output: SurfaceTexture = surface
                    .get_current_texture()
                    .expect("Could not get a texture to draw on!");

                // Handle to the TextureView object which describes the texture and related metadata
                // https://docs.rs/wgpu/latest/wgpu/struct.TextureView.html
                let view: TextureView = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                // Used for encoding instructions to the GPU
                // https://docs.rs/wgpu/latest/wgpu/struct.CommandEncoder.html
                let mut encoder: CommandEncoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

                // Command to render
                // https://docs.rs/wgpu/latest/wgpu/struct.RenderPassDescriptor.html
                let render_pass_descriptor: RenderPassDescriptor = wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        // Royal Purple - 104, 71, 141
                        // TODO: Fix so color is shown accurately
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 104.0 / 255.0,
                                g: 71.0 / 255.0,
                                b: 141.0 / 255.0,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                };

                // Block expression for ending the borrow of encoder
                // https://doc.rust-lang.org/reference/expressions/block-expr.html
                // https://doc.rust-lang.org/beta/rust-by-example/scope/borrow.html
                {
                    // Render command
                    // https://docs.rs/wgpu/latest/wgpu/struct.RenderPass.html
                    let _render_pass: RenderPass =
                        encoder.begin_render_pass(&render_pass_descriptor);
                }

                queue.submit(core::iter::once(encoder.finish()));
                output.present();
            }

            // Last event to ever be executed on shutdown
            Event::LoopExiting => {
                // Should i use this?
            }

            // All unnamed events
            _ => (),
        }
    });

    Ok(())
}
