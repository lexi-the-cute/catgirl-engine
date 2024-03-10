use std::sync::Arc;

use winit::{dpi::PhysicalSize, window::Window};

use wgpu::{Adapter, Device, DeviceDescriptor, Instance, Queue, Surface};

/// Struct used for storing the state of a window
#[derive(Debug)]
pub struct WindowState<'a> {
    /// Handle to the window which holds the drawable surface
    pub window: Arc<Window>,

    /// Context for WGPU objects
    pub instance: Instance,

    /// Handle to the graphics device (e.g. the gpu)
    pub adapter: Option<Adapter>,

    /// The surface on which to draw graphics on
    pub surface: Surface<'a>,

    /// Connection to the graphics device provided by the adapter
    pub device: Option<Device>,

    /// Queue in which to send commands to the graphics device
    pub queue: Option<Queue>,
}

impl WindowState<'_> {
    /// Used to initialize a new window and setup the graphics
    pub fn new(window: Window) -> Self {
        let window_arc: Arc<Window> = Arc::new(window);

        // Context for all WGPU objects
        // https://docs.rs/wgpu/latest/wgpu/struct.Instance.html
        debug!("Creating wgpu instance...");
        let instance: Instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

        debug!("Creating wgpu surface...");
        let surface: Surface<'_> = instance
            .create_surface(window_arc.clone())
            .expect("Could not create surface!");

        Self {
            window: window_arc,
            instance,
            surface,
            adapter: None,
            device: None,
            queue: None,
        }
    }

    /// Used to retrieve the best limits for the target
    fn get_limits(&self) -> wgpu::Limits {
        if cfg!(any(target_os = "android", target_os = "ios")) {
            wgpu::Limits {
                max_texture_dimension_1d: 4096,
                max_texture_dimension_2d: 4096,
                ..Default::default()
            }
        } else if cfg!(target_family = "wasm") {
            wgpu::Limits {
                max_compute_workgroups_per_dimension: 0,
                max_compute_workgroup_size_z: 0,
                max_compute_workgroup_size_y: 0,
                max_compute_workgroup_size_x: 0,
                max_compute_invocations_per_workgroup: 0,
                max_compute_workgroup_storage_size: 0,
                max_storage_buffer_binding_size: 0,
                max_storage_textures_per_shader_stage: 0,
                max_storage_buffers_per_shader_stage: 0,
                max_dynamic_storage_buffers_per_pipeline_layout: 0,
                ..Default::default()
            }
        } else {
            wgpu::Limits::default()
        }
    }

    /// Initalize the async graphics portion of the window state
    pub async fn initialize_graphics(&mut self) {
        // Describe's a device
        // For use with adapter's request device
        // https://docs.rs/wgpu/latest/wgpu/type.DeviceDescriptor.html
        debug!("Describing wgpu device...");
        let mut device_descriptor: DeviceDescriptor = wgpu::DeviceDescriptor::default();

        // Set limits to make this run on more devices
        // TODO: Research how to dynamically set limits for the running device
        debug!("Setting WGPU limits...");

        let limits: wgpu::Limits = self.get_limits();
        device_descriptor.required_limits = limits;

        // Handle to graphics device (e.g. GPU)
        // https://docs.rs/wgpu/latest/wgpu/struct.Adapter.html
        // https://crates.io/crates/futures
        debug!("Grabbing wgpu adapter...");
        let adapter_future = self
            .instance
            .request_adapter(&wgpu::RequestAdapterOptions::default());
        self.adapter = Some(adapter_future.await.expect("Could not grab WGPU adapter!"));

        // Opens a connection to the graphics device (e.g. GPU)
        debug!("Opening connection with graphics device (e.g. GPU)...");
        let device_future = self
            .adapter
            .as_ref()
            .unwrap()
            .request_device(&device_descriptor, None);

        let (device, queue) = device_future
            .await
            .expect("Could not open a connection with the graphics device!");
        self.device = Some(device);
        self.queue = Some(queue);

        let size: PhysicalSize<u32> = if cfg!(target_family = "wasm") {
            PhysicalSize::new(400, 100)
        } else {
            self.window.clone().inner_size()
        };

        trace!(
            "Window inner size (Initialize Graphics): ({}, {})",
            size.width,
            size.height
        );
        self.surface.configure(
            self.device.as_ref().unwrap(),
            &self
                .surface
                .get_default_config(self.adapter.as_ref().unwrap(), size.width, size.height)
                .expect("Could not get surface default config!"),
        );
    }

    /// Recreate the surface after it has been destroyed (e.g. used on Android)
    pub fn recreate_surface(&mut self) {
        if self.device.is_none() {
            warn!("Device is not setup... Have graphics been initialized?");
            return;
        }

        if self.adapter.is_none() {
            warn!("Adapter is not setup... Have graphics been initialized?");
            return;
        }

        // Handle to the surface on which to draw on (e.g. a window)
        // https://docs.rs/wgpu/latest/wgpu/struct.Surface.html
        debug!("Creating wgpu surface...");
        let surface: Surface<'_> = self
            .instance
            .create_surface(self.window.clone())
            .expect("Could not create surface!");

        let size: PhysicalSize<u32> = self.window.clone().inner_size();
        surface.configure(
            self.device.as_ref().unwrap(),
            &surface
                .get_default_config(self.adapter.as_ref().unwrap(), size.width, size.height)
                .expect("Could not get surface default config!"),
        );

        self.surface = surface;
    }
}
