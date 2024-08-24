#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
//! Rotating a globe using the best performance techniques
//!
//! In general the bottlekneck is the bulk transfer of data from CPU to GPU.
//!
//! Perviously the best in class still resulted in transfers was via javascript ( Path2d )
//! This had the limited benefit of reducing the number of browser/system calls
//!
//! Bulk transfer here is more direct without javascript.
//!
//! This application is based on a [wgpu/examples/hello_triangle](https://github.com/gfx-rs/wgpu/blob/trunk/examples/src/hello_triangle/mod.rs)

extern crate d3_geo_rs;
extern crate rwh_06;

#[cfg(not(any(android_platform, ios_platform)))]
extern crate softbuffer;

pub(crate) mod app;
pub(crate) mod windows_state;

use core::fmt;
use core::fmt::Debug;
use std::error::Error;

use app::Application;
// use geo_types::Coord;
// use geo_types::Geometry;
// use wgpu::IndexFormat;
// use wgpu::PipelineCompilationOptions;
// use wgpu::Surface;

use ::tracing::info;
use winit::event::MouseButton;
use winit::event_loop::EventLoop;
use winit::keyboard::ModifiersState;
use winit::window::CursorIcon;
use winit::window::CustomCursor;
use winit::window::CustomCursorSource;
use winit::window::Icon;
use winit::window::Theme;

// use d3_geo_rs::graticule::generate_mls;
// use d3_geo_rs::path::builder::Builder as PathBuilder;
// use d3_geo_rs::path::wgpu::polylines::PolyLines as PolyLinesWGPU;
// use d3_geo_rs::path::wgpu::Vertex;
// use d3_geo_rs::projection::orthographic::Orthographic;
// use d3_geo_rs::projection::Build;
// use d3_geo_rs::projection::RawBase;
// use d3_geo_rs::projection::ScaleSet;
// use d3_geo_rs::projection::TranslateSet;

/// The amount of points to around the window for drag resize direction calculations.
const BORDER_SIZE: f64 = 20.;

fn modifiers_to_string(mods: ModifiersState) -> String {
    let mut mods_line = String::new();
    // Always add + since it's printed as a part of the bindings.
    for (modifier, desc) in [
        (ModifiersState::SUPER, "Super+"),
        (ModifiersState::ALT, "Alt+"),
        (ModifiersState::CONTROL, "Ctrl+"),
        (ModifiersState::SHIFT, "Shift+"),
    ] {
        if !mods.contains(modifier) {
            continue;
        }

        mods_line.push_str(desc);
    }
    mods_line
}

// async fn run(event_loop: EventLoop<()>, window: Window) {
//     let mut size = window.inner_size();
//     size.width = size.width.max(1);
//     size.height = size.height.max(1);

//     let instance = wgpu::Instance::default();

//     let surface = instance.create_surface(&window).unwrap();
//     let adapter = instance
//         .request_adapter(&wgpu::RequestAdapterOptions {
//             power_preference: wgpu::PowerPreference::default(),
//             force_fallback_adapter: false,
//             // Request an adapter which can render to our surface
//             compatible_surface: Some(&surface),
//         })
//         .await
//         .expect("Failed to find an appropriate adapter");

//     // Create the logical device and command queue
//     let (device, _queue) = adapter
//         .request_device(
//             &wgpu::DeviceDescriptor {
//                 label: None,
//                 required_features: wgpu::Features::empty(),
//                 // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
//                 required_limits: wgpu::Limits::downlevel_webgl2_defaults()
//                     .using_resolution(adapter.limits()),
//                 memory_hints: wgpu::MemoryHints::MemoryUsage,
//             },
//             None,
//         )
//         .await
//         .expect("Failed to create device");

//     // Load the shaders from disk
//     let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
//         label: None,
//         source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
//             "../shader.wgsl"
//         ))),
//     });

//     let pipeline_layout =
//         device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//             label: None,
//             bind_group_layouts: &[],
//             push_constant_ranges: &[],
//         });

//     let swapchain_capabilities = surface.get_capabilities(&adapter);
//     let swapchain_format = swapchain_capabilities.formats[0];

//     let mut projector_builder = Orthographic::builder::<PolyLinesWGPU>();
//     projector_builder
//         .scale_set(800_f32 / 1.3_f32 / std::f32::consts::PI)
//         .translate_set(&Coord { x: 0_f32, y: 0_f32 });

//     // Graticule
//     let graticule: Geometry<f32> = generate_mls();
//     // println!("graticule {:#?}", &graticule);

//     let projector = projector_builder.build();

//     let endpoint = PolyLinesWGPU::default();
//     let path_builder = PathBuilder::new(endpoint);
//     let mut path = path_builder.build(projector);

//     let (verticies, indicies) = path.object(&graticule);

//     let mut minx = f32::MAX;
//     let mut maxx = f32::MIN;
//     let mut miny = f32::MAX;
//     let mut maxy = f32::MIN;
//     for v in &verticies {
//         if v.pos[0] < minx {
//             minx = v.pos[0];
//         }
//         if v.pos[0] > maxx {
//             maxx = v.pos[0];
//         }

//         if v.pos[1] < miny {
//             miny = v.pos[1];
//         }
//         if v.pos[1] > maxy {
//             maxy = v.pos[1];
//         }
//     }
//     // println!("x: min{minx}, max{maxx}");
//     // println!("y: min{miny}, max{maxy}");

//     // println!("indicies: {:#?}", &indicies);
//     // println!("vertcies: {:#?}", &verticies);

//     let render_pipeline =
//         device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
//             label: None,
//             layout: Some(&pipeline_layout),
//             vertex: wgpu::VertexState {
//                 module: &shader,
//                 entry_point: "vs_main",
//                 buffers: &[Vertex::desc()],
//                 compilation_options: PipelineCompilationOptions::default(),
//             },
//             fragment: Some(wgpu::FragmentState {
//                 module: &shader,
//                 entry_point: "fs_main",
//                 compilation_options: PipelineCompilationOptions::default(),
//                 targets: &[Some(swapchain_format.into())],
//             }),
//             primitive: wgpu::PrimitiveState {
//                 topology: wgpu::PrimitiveTopology::LineStrip,
//                 // Enables "PRIMITIVE_RESTART" mode
//                 // see `rust_d3_geo::path::wgpu::PRIMITIVE_RESTART_TOKEN`
//                 strip_index_format: Some(IndexFormat::Uint32),
//                 ..Default::default()
//             },
//             depth_stencil: None,
//             multisample: wgpu::MultisampleState::default(),
//             multiview: None,
//             cache: None,
//         });

//     let mut config = surface
//         .get_default_config(&adapter, size.width, size.height)
//         .unwrap();
//     surface.configure(&device, &config);

//     // let window = &window;
//     // event_loop
//     //     .run_app(move |event, target| {
//     //         // Have the closure take ownership of the resources.
//     //         // `event_loop.run` never returns, therefore we must do this to ensure
//     //         // the resources are properly cleaned up.
//     //         let _ = (&instance, &adapter, &shader, &pipeline_layout);

//     //         if let Event::WindowEvent {
//     //             window_id: _,
//     //             event,
//     //         } = event
//     //         {
//     //             match event {
//     //                 WindowEvent::Resized(new_size) => {
//     //                     // Reconfigure the surface with the new size
//     //                     config.width = new_size.width.max(1);
//     //                     config.height = new_size.height.max(1);
//     //                     surface.configure(&device, &config);
//     //                     // On macos the window needs to be redrawn manually after resizing
//     //                     window.request_redraw();
//     //                 }
//     //                 WindowEvent::RedrawRequested => {
//     //                     let frame = surface.get_current_texture().expect(
//     //                         "Failed to acquire next swap chain texture",
//     //                     );
//     //                     let view = frame.texture.create_view(
//     //                         &wgpu::TextureViewDescriptor::default(),
//     //                     );
//     //                     let mut encoder = device.create_command_encoder(
//     //                         &wgpu::CommandEncoderDescriptor { label: None },
//     //                     );

//     //                     let vertex_buffer = device.create_buffer_init(
//     //                         &wgpu::util::BufferInitDescriptor {
//     //                             label: Some("Points"),
//     //                             contents: bytemuck::cast_slice(&verticies),
//     //                             usage: wgpu::BufferUsages::VERTEX,
//     //                         },
//     //                     );

//     //                     let index_buffer = device.create_buffer_init(
//     //                         &wgpu::util::BufferInitDescriptor {
//     //                             label: Some("Index buffer"),
//     //                             contents: bytemuck::cast_slice(&indicies),
//     //                             usage: wgpu::BufferUsages::INDEX,
//     //                         },
//     //                     );

//     //                     {
//     //                         let mut rpass = encoder.begin_render_pass(
//     //                             &wgpu::RenderPassDescriptor {
//     //                                 label: None,
//     //                                 color_attachments: &[Some(
//     //                                     wgpu::RenderPassColorAttachment {
//     //                                         view: &view,
//     //                                         resolve_target: None,
//     //                                         ops: wgpu::Operations {
//     //                                             load: wgpu::LoadOp::Clear(
//     //                                                 wgpu::Color::BLACK,
//     //                                             ),
//     //                                             store: wgpu::StoreOp::Store,
//     //                                         },
//     //                                     },
//     //                                 )],
//     //                                 depth_stencil_attachment: None,
//     //                                 timestamp_writes: None,
//     //                                 occlusion_query_set: None,
//     //                             },
//     //                         );
//     //                         rpass.set_pipeline(&render_pipeline);
//     //                         rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
//     //                         rpass.set_index_buffer(
//     //                             index_buffer.slice(..),
//     //                             wgpu::IndexFormat::Uint32,
//     //                         );
//     //                         // instances 0..1 implies instancing is not being used!!!.
//     //                         rpass.draw_indexed(
//     //                             0..indicies.len() as u32,
//     //                             0,
//     //                             0..1,
//     //                         );
//     //                     }

//     //                     queue.submit(Some(encoder.finish()));
//     //                     frame.present();
//     //                 }
//     //                 WindowEvent::CloseRequested => target.exit(),
//     //                 _ => {}
//     //             };
//     //         }
//     //     })
//     //     .unwrap();
// }

#[path = "util/tracing.rs"]
mod tracing;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum UserEvent {
    WakeUp,
}

/// Entry point
///
/// Initialisation before the APP starts.
///
/// Logging
///
/// Environment sensing
///  eg. check build for HTML canvas if required.
fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(web_platform)]
    console_error_panic_hook::set_once();

    tracing::init();

    let event_loop = EventLoop::<UserEvent>::with_user_event().build()?;
    let _event_loop_proxy = event_loop.create_proxy();

    // Wire the user event from another thread.
    #[cfg(not(web_platform))]
    std::thread::spawn(move || {
        // Wake up the `event_loop` once every second and dispatch a custom event
        // from a different thread.
        info!("Starting to send user event every second");
        loop {
            let _ = _event_loop_proxy.send_event(UserEvent::WakeUp);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    let mut state = Application::new(&event_loop);

    event_loop.run_app(&mut state).map_err(Into::into)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Action {
    CloseWindow,
    ToggleCursorVisibility,
    CreateNewWindow,
    ToggleResizeIncrements,
    ToggleImeInput,
    ToggleDecorations,
    ToggleResizable,
    ToggleFullscreen,
    ToggleMaximize,
    Minimize,
    NextCursor,
    NextCustomCursor,
    #[cfg(web_platform)]
    UrlCustomCursor,
    #[cfg(web_platform)]
    AnimationCustomCursor,
    CycleCursorGrab,
    PrintHelp,
    DragWindow,
    DragResizeWindow,
    ShowWindowMenu,
    #[cfg(macos_platform)]
    CycleOptionAsAlt,
    SetTheme(Option<Theme>),
    #[cfg(macos_platform)]
    CreateNewTab,
    RequestResize,
}

impl Action {
    const fn help(self) -> &'static str {
        match self {
            Self::CloseWindow => "Close window",
            Self::ToggleCursorVisibility => "Hide cursor",
            Self::CreateNewWindow => "Create new window",
            Self::ToggleImeInput => "Toggle IME input",
            Self::ToggleDecorations => "Toggle decorations",
            Self::ToggleResizable => "Toggle window resizable state",
            Self::ToggleFullscreen => "Toggle fullscreen",
            Self::ToggleMaximize => "Maximize",
            Self::Minimize => "Minimize",
            Self::ToggleResizeIncrements => {
                "Use resize increments when resizing window"
            }
            Self::NextCursor => "Advance the cursor to the next value",
            Self::NextCustomCursor => "Advance custom cursor to the next value",
            #[cfg(web_platform)]
            Action::UrlCustomCursor => "Custom cursor from an URL",
            #[cfg(web_platform)]
            Action::AnimationCustomCursor => "Custom cursor from an animation",
            Self::CycleCursorGrab => "Cycle through cursor grab mode",
            Self::PrintHelp => "Print help",
            Self::DragWindow => "Start window drag",
            Self::DragResizeWindow => "Start window drag-resize",
            Self::ShowWindowMenu => "Show window menu",
            #[cfg(macos_platform)]
            Action::CycleOptionAsAlt => "Cycle option as alt mode",
            Self::SetTheme(None) => "Change to the system theme",
            Self::SetTheme(Some(Theme::Light)) => "Change to a light theme",
            Self::SetTheme(Some(Theme::Dark)) => "Change to a dark theme",
            #[cfg(macos_platform)]
            Self::CreateNewTab => "Create new tab",
            Self::RequestResize => "Request a resize",
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}

fn decode_cursor(bytes: &[u8]) -> CustomCursorSource {
    let img = image::load_from_memory(bytes).unwrap().to_rgba8();
    let samples = img.into_flat_samples();
    let (_, w, h) = samples.extents();
    let (w, h) = (w as u16, h as u16);
    CustomCursor::from_rgba(samples.samples, w, h, w / 2, h / 2).unwrap()
}

fn mouse_button_to_string(button: MouseButton) -> &'static str {
    match button {
        MouseButton::Left => "LMB",
        MouseButton::Right => "RMB",
        MouseButton::Middle => "MMB",
        MouseButton::Back => "Back",
        MouseButton::Forward => "Forward",
        MouseButton::Other(_) => "",
    }
}

#[cfg(web_platform)]
fn url_custom_cursor() -> CustomCursorSource {
    use std::sync::atomic::{AtomicU64, Ordering};

    static URL_COUNTER: AtomicU64 = AtomicU64::new(0);

    CustomCursor::from_url(
        format!(
            "https://picsum.photos/128?random={}",
            URL_COUNTER.fetch_add(1, Ordering::Relaxed)
        ),
        64,
        64,
    )
}

struct Binding<T: Eq> {
    trigger: T,
    mods: ModifiersState,
    action: Action,
}

impl<T: Eq> Binding<T> {
    const fn new(trigger: T, mods: ModifiersState, action: Action) -> Self {
        Self {
            trigger,
            mods,
            action,
        }
    }

    fn is_triggered_by(&self, trigger: &T, mods: &ModifiersState) -> bool {
        &self.trigger == trigger && &self.mods == mods
    }
}

/// Cursor list to cycle through.
const CURSORS: &[CursorIcon] = &[
    CursorIcon::Default,
    CursorIcon::Crosshair,
    CursorIcon::Pointer,
    CursorIcon::Move,
    CursorIcon::Text,
    CursorIcon::Wait,
    CursorIcon::Help,
    CursorIcon::Progress,
    CursorIcon::NotAllowed,
    CursorIcon::ContextMenu,
    CursorIcon::Cell,
    CursorIcon::VerticalText,
    CursorIcon::Alias,
    CursorIcon::Copy,
    CursorIcon::NoDrop,
    CursorIcon::Grab,
    CursorIcon::Grabbing,
    CursorIcon::AllScroll,
    CursorIcon::ZoomIn,
    CursorIcon::ZoomOut,
    CursorIcon::EResize,
    CursorIcon::NResize,
    CursorIcon::NeResize,
    CursorIcon::NwResize,
    CursorIcon::SResize,
    CursorIcon::SeResize,
    CursorIcon::SwResize,
    CursorIcon::WResize,
    CursorIcon::EwResize,
    CursorIcon::NsResize,
    CursorIcon::NeswResize,
    CursorIcon::NwseResize,
    CursorIcon::ColResize,
    CursorIcon::RowResize,
];

fn load_icon(bytes: &[u8]) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(bytes).unwrap().into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height)
        .expect("Failed to open icon")
}
