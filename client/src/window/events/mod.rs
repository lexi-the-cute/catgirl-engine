use wgpu::{
    Adapter, CommandEncoder, Device, Queue, RenderPass, RenderPassDescriptor, Surface,
    SurfaceTexture, TextureView,
};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyEvent, MouseButton},
    event_loop::EventLoopWindowTarget,
    keyboard::NamedKey,
    window::{Window, WindowBuilder},
};

use crate::window::window_state::WindowState;

/// The close button was pressed. Usually on the top right corner
pub(crate) fn close_requested(window_target: &EventLoopWindowTarget<()>) {
    debug!("The Close Button Was Pressed! Stopping...");
    window_target.exit();
}

/// This is technically not an event, but is called by the Resume event
pub(crate) async fn create_window(
    window_target: &EventLoopWindowTarget<()>,
) -> WindowState<'static> {
    debug!("Creating window...");
    let window = WindowBuilder::new()
        .with_title("Catgirl Engine")
        .with_window_icon(Some(crate::get_icon()))
        .build(window_target)
        .expect("Could not create window!");

    WindowState::new(window).await
}

/// Resumed window after first resume call (e.g Android)
pub(crate) fn resumed_window(window_state: &mut WindowState) {
    debug!("Resuming application...");

    window_state.recreate_surface();
}

/// Suspended window
pub(crate) fn suspended_window() {
    debug!("Suspending application...");
}

// TODO: Offload to separate function with key mapping config
// processInput()
// update() - Input gets passed to (internal) server, physics gets passed back
/// Key was pressed on keyboard
pub(crate) fn pressed_key(event: KeyEvent, window_target: &EventLoopWindowTarget<()>) {
    match event.logical_key {
        winit::keyboard::Key::Named(NamedKey::Escape) => {
            debug!("The Escape Key Was Pressed! Stopping...");
            window_target.exit();
        }
        _ => {
            debug!("Event: {:#?}", event);
        }
    }
}

/// Mouse was clicked
pub(crate) fn clicked_mouse(
    state: ElementState,
    button: MouseButton,
    _window_target: &EventLoopWindowTarget<()>,
) {
    if state.is_pressed() {
        debug!("Mouse {:?} was pressed...", button);
    } else {
        debug!("Mouse {:?} was released...", button);
    }
}

/// The window was resized
pub(crate) fn resized_window(window_state: &WindowState, _size: PhysicalSize<u32>) {
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

/// The window was either just focused or lost focus
pub(crate) fn changed_focus(focused: bool) {
    trace!("Window focused: {focused}");
}

/// Redraw surface
pub(crate) fn requested_redraw(window_state: &WindowState) {
    // TODO: https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/#what-s-a-pipeline
    // Configure a surface for drawing on
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

    // Color to render
    // Royal Purple - 104, 71, 141
    let (r, g, b) = crate::render::srgb_to_linear_srgb(104, 71, 141);

    // Command to render
    // https://docs.rs/wgpu/latest/wgpu/struct.RenderPassDescriptor.html
    let render_pass_descriptor: RenderPassDescriptor = wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color { r, g, b, a: 1.0 }),
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
        let _render_pass: RenderPass = encoder.begin_render_pass(&render_pass_descriptor);
    }

    queue.submit(core::iter::once(encoder.finish()));
    output.present();
}

/// Exiting loop
pub(crate) fn exiting_loop() {
    trace!("Winit loop is exiting...");
}

/// New events have arrived
pub(crate) fn new_events() {
    // Currently only exists to remove from unhandled_event logging
}

/// About to wait for new events to arrive
pub(crate) fn about_to_wait_event() {
    // Currently only exists to remove from unhandled_event logging
}

/// Catches previously unhandled events
pub(crate) fn unhandled_event(event: Event<()>) {
    trace!("Unhandled event: {:?}", event);
}
