

use std::{borrow::Cow, collections::HashMap, sync::Arc};
use env_logger::builder;
use wgpu::{Adapter, Device, PipelineLayout, RenderPipeline, ShaderModule, Surface, SurfaceConfiguration};
use winit::{
    application::ApplicationHandler, dpi::PhysicalPosition, event::{Event, WindowEvent}, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, keyboard::ModifiersState, window::{CursorGrabMode, Window}
};
use std::error::Error;
use winit::window::{ WindowId};
use crate::windows_state::WindowState;

use tracing::info;

pub(crate) struct App<'a>{
  // mode: Mode,
  // request_redraw: bool,
  // wait_cancelled: bool,
  // close_requested: bool,
  // window: Option<Window>
  pub(crate) windows: HashMap<WindowId, WindowState<'a>>,
  pub(crate) adapter: Adapter,
  pub(crate) config: SurfaceConfiguration,
  pub(crate) device: Device,
  pub(crate) instance: wgpu::Instance,
  pub(crate) pipeline_layout: PipelineLayout,
  pub(crate) render_pipeline: RenderPipeline,
  pub(crate) shader: ShaderModule,
  pub(crate) surface: Surface<'a>,
}

impl <'a>  App<'a>{
pub(crate) async fn new() -> Self{

  // let window_attributes = Window::default_attributes().with_title("A rotating globe");
  // let window = event_loop.create_window(window_attributes).unwrap();


  // let mut size = window.inner_size();
  // size.width = size.width.max(1);
  // size.height = size.height.max(1);

let instance = wgpu::Instance::default();

let surface = instance.create_surface(&window).unwrap();
let adapter = instance
    .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        // Request an adapter which can render to our surface
        compatible_surface: Some(&surface),
    })
    .await
    .expect("Failed to find an appropriate adapter");

// Create the logical device and command queue
let (device, queue) = adapter
    .request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
            required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                .using_resolution(adapter.limits()),
            memory_hints: wgpu::MemoryHints::MemoryUsage,
        },
        None,
    )
    .await
    .expect("Failed to create device");

// Load the shaders from disk
let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: None,
    source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../shader.wgsl"))),
});

let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: None,
    bind_group_layouts: &[],
    push_constant_ranges: &[],
});

let swapchain_capabilities = surface.get_capabilities(&adapter);
let swapchain_format = swapchain_capabilities.formats[0];

let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    label: None,
    layout: Some(&pipeline_layout),
    vertex: wgpu::VertexState {
        module: &shader,
        entry_point: "vs_main",
        buffers: &[],
        compilation_options: Default::default(),
    },
    fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: "fs_main",
        compilation_options: Default::default(),
        targets: &[Some(swapchain_format.into())],
    }),
    primitive: wgpu::PrimitiveState::default(),
    depth_stencil: None,
    multisample: wgpu::MultisampleState::default(),
    multiview: None,
    cache: None,
});

let mut config = surface
    .get_default_config(&adapter, size.width, size.height)
    .unwrap();
surface.configure(&device, &config);
Self{
  adapter,
  config,
  device,
  instance,
  pipeline_layout,
  render_pipeline,
  shader,
  surface,
  windows: Default::default()
}
}

fn create_window(
&mut self,
event_loop: &ActiveEventLoop,
_tab_id: Option<String>,
) -> Result<WindowId, Box<dyn Error>> {
// TODO read-out activation token.

#[allow(unused_mut)]
let mut window_attributes = Window::default_attributes()
    .with_title("Globe")
    .with_transparent(true)
    .with_window_icon(Some(self.icon.clone()));

#[cfg(any(x11_platform, wayland_platform))]
if let Some(token) = event_loop.read_token_from_env() {
    startup_notify::reset_activation_token_env();
    info!("Using token {:?} to activate a window", token);
    window_attributes = window_attributes.with_activation_token(token);
}

#[cfg(macos_platform)]
if let Some(tab_id) = _tab_id {
    window_attributes = window_attributes.with_tabbing_identifier(&tab_id);
}

#[cfg(web_platform)]
{
    window_attributes = window_attributes.with_append(true);
}

let window = event_loop.create_window(window_attributes)?;

#[cfg(ios_platform)]
{
    use winit::platform::ios::WindowExtIOS;
    window.recognize_doubletap_gesture(true);
    window.recognize_pinch_gesture(true);
    window.recognize_rotation_gesture(true);
    window.recognize_pan_gesture(true, 2, 2);
}

let window_state = WindowState::new(self, window)?;
let window_id = window_state.window.id();
info!("Created new window with id={window_id:?}");
self.windows.insert(window_id, window_state);
Ok(window_id)
}

}
