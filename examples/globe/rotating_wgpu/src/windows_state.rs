use core::mem;
use std::borrow::Cow;
use std::error::Error;
use std::sync::Arc;

use d3_geo_rs::graticule::generate_mls;
use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::path::wgpu::polylines::Index;
use d3_geo_rs::path::wgpu::polylines::PolyLines as PolyLinesWGPU;
use d3_geo_rs::path::wgpu::Vertex;
use d3_geo_rs::projection::orthographic::Orthographic;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::RawBase;
use d3_geo_rs::projection::ScaleSet;
use d3_geo_rs::projection::TranslateSet;
use geo_types::Coord;
use geo_types::Geometry;
use pollster::FutureExt;
use wgpu::util::DeviceExt;
use wgpu::IndexFormat;
use wgpu::PipelineCompilationOptions;
use wgpu::Queue;
use winit::{
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize},
    keyboard::ModifiersState,
    window::{
        Cursor, CursorGrabMode, CustomCursor, Fullscreen, ResizeDirection,
        Theme, Window,
    },
};

use ::tracing::{error, info};

use crate::{app::Application, BORDER_SIZE, CURSORS};

/// State of the window.
pub(crate) struct WindowState<'a> {
    /// IME input.
    ime: bool,

    // /// Render surface.
    // ///
    // /// NOTE: This surface must be dropped before the `Window`.
    // #[cfg(not(any(android_platform, ios_platform)))]
    // surface: Surface<DisplayHandle<'static>, Arc<Window>>,
    /// The actual winit Window.
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    render_pipeline: wgpu::RenderPipeline,
    indicies: Vec<Index>,
    queue: Queue,
    verticies: Vec<Vertex>,
    // device_pipeline: wgpu::DevicePipeline,
    pub(crate) window: Arc<Window>,
    /// The window theme we're drawing with.
    theme: Theme,
    /// Cursor position over the window.
    cursor_position: Option<PhysicalPosition<f64>>,
    /// Window modifiers state.
    pub(crate) modifiers: ModifiersState,
    /// Occlusion state of the window.
    occluded: bool,
    /// Current cursor grab mode.
    cursor_grab: CursorGrabMode,
    /// The amount of zoom into window.
    pub(crate) zoom: f64,
    /// The amount of rotation of the window.
    pub(crate) rotated: f32,
    /// The amount of pan of the window.
    pub(crate) panned: PhysicalPosition<f32>,

    // Cursor states.
    named_idx: usize,
    custom_idx: usize,
    cursor_hidden: bool,
}

impl<'a> WindowState<'a> {
    pub(crate) fn new(
        app: &Application<'a>,
        window: Window,
    ) -> Result<Self, Box<dyn Error>> {
        let window = Arc::new(window);

        // // SAFETY: the surface is dropped before the `window` which provided it with handle, thus
        // // it doesn't outlive it.
        // #[cfg(not(any(android_platform, ios_platform)))]
        // let surface =
        //     Surface::new(app.context.as_ref().unwrap(), Arc::clone(&window))?;

        let theme = window.theme().unwrap_or(Theme::Dark);
        info!("Theme: {theme:?}");
        let named_idx = 0;
        window.set_cursor(CURSORS[named_idx]);

        // Allow IME out of the box.
        let ime = true;
        window.set_ime_allowed(ime);

        let size = window.inner_size();

        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .block_on()
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
            .block_on()
            .expect("Failed to create device");

        // Load the shaders from disk
        let shader =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                    "../shader.wgsl"
                ))),
            });

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let mut projector_builder = Orthographic::builder::<PolyLinesWGPU>();
        projector_builder
            .scale_set(800_f32 / 1.3_f32 / std::f32::consts::PI)
            .translate_set(&Coord { x: 0_f32, y: 0_f32 });

        // Graticule
        let graticule: Geometry<f32> = generate_mls();
        // println!("graticule {:#?}", &graticule);

        let projector = projector_builder.build();

        let endpoint = PolyLinesWGPU::default();
        let path_builder = PathBuilder::new(endpoint);
        let mut path = path_builder.build(projector);

        let (verticies, indicies) = path.object(&graticule);

        let mut minx = f32::MAX;
        let mut maxx = f32::MIN;
        let mut miny = f32::MAX;
        let mut maxy = f32::MIN;
        for v in &verticies {
            if v.pos[0] < minx {
                minx = v.pos[0];
            }
            if v.pos[0] > maxx {
                maxx = v.pos[0];
            }

            if v.pos[1] < miny {
                miny = v.pos[1];
            }
            if v.pos[1] > maxy {
                maxy = v.pos[1];
            }
        }
        // println!("x: min{minx}, max{maxx}");
        // println!("y: min{miny}, max{maxy}");

        // println!("indicies: {:#?}", &indicies);
        // println!("vertcies: {:#?}", &verticies);

        let render_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                    compilation_options: PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    compilation_options: PipelineCompilationOptions::default(),
                    targets: &[Some(swapchain_format.into())],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::LineStrip,
                    // Enables "PRIMITIVE_RESTART" mode
                    // see `rust_d3_geo::path::wgpu::PRIMITIVE_RESTART_TOKEN`
                    strip_index_format: Some(IndexFormat::Uint32),
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface.configure(&device, &config);

        let mut state = Self {
            #[cfg(macos_platform)]
            option_as_alt: window.option_as_alt(),
            custom_idx: app.custom_cursors.len() - 1,
            cursor_grab: CursorGrabMode::None,
            named_idx,
            // #[cfg(not(any(android_platform, ios_platform)))]
            // surface,
            surface,
            device,
            render_pipeline,
            indicies,
            queue,
            verticies,
            window,
            theme,
            ime,
            cursor_position: Option::default(),
            cursor_hidden: Default::default(),
            modifiers: ModifiersState::default(),
            occluded: Default::default(),
            rotated: Default::default(),
            panned: PhysicalPosition::default(),
            zoom: Default::default(),
        };

        state.resize(size);
        Ok(state)
    }
    pub fn toggle_ime(&mut self) {
        self.ime = !self.ime;
        self.window.set_ime_allowed(self.ime);
        if let Some(position) =
            self.ime.then_some(self.cursor_position).flatten()
        {
            self.window
                .set_ime_cursor_area(position, PhysicalSize::new(20, 20));
        }
    }

    pub fn minimize(&self) {
        self.window.set_minimized(true);
    }

    pub fn cursor_moved(&mut self, position: PhysicalPosition<f64>) {
        self.cursor_position = Some(position);
        if self.ime {
            self.window
                .set_ime_cursor_area(position, PhysicalSize::new(20, 20));
        }
    }

    pub fn cursor_left(&mut self) {
        self.cursor_position = None;
    }

    /// Toggle maximized.
    pub(crate) fn toggle_maximize(&self) {
        let maximized = self.window.is_maximized();
        self.window.set_maximized(!maximized);
    }

    /// Toggle window decorations.
    pub(crate) fn toggle_decorations(&self) {
        let decorated = self.window.is_decorated();
        self.window.set_decorations(!decorated);
    }

    /// Toggle window resizable state.
    pub(crate) fn toggle_resizable(&self) {
        let resizable = self.window.is_resizable();
        self.window.set_resizable(!resizable);
    }

    /// Toggle cursor visibility
    pub(crate) fn toggle_cursor_visibility(&mut self) {
        self.cursor_hidden = !self.cursor_hidden;
        self.window.set_cursor_visible(!self.cursor_hidden);
    }

    /// Toggle resize increments on a window.
    pub(crate) fn toggle_resize_increments(&self) {
        let new_increments = match self.window.resize_increments() {
            Some(_) => None,
            None => Some(LogicalSize::new(25.0, 25.0)),
        };
        info!("Had increments: {}", new_increments.is_none());
        self.window.set_resize_increments(new_increments);
    }

    /// Toggle fullscreen.
    pub(crate) fn toggle_fullscreen(&self) {
        let fullscreen = if self.window.fullscreen().is_some() {
            None
        } else {
            Some(Fullscreen::Borderless(None))
        };

        self.window.set_fullscreen(fullscreen);
    }

    /// Cycle through the grab modes ignoring errors.
    pub(crate) fn cycle_cursor_grab(&mut self) {
        self.cursor_grab = match self.cursor_grab {
            CursorGrabMode::None => CursorGrabMode::Confined,
            CursorGrabMode::Confined => CursorGrabMode::Locked,
            CursorGrabMode::Locked => CursorGrabMode::None,
        };
        info!("Changing cursor grab mode to {:?}", self.cursor_grab);
        if let Err(err) = self.window.set_cursor_grab(self.cursor_grab) {
            error!("Error setting cursor grab: {err}");
        }
    }

    #[cfg(macos_platform)]
    pub(crate) fn cycle_option_as_alt(&mut self) {
        use winit::platform::macos::OptionAsAlt;
        self.option_as_alt = match self.option_as_alt {
            OptionAsAlt::None => OptionAsAlt::OnlyLeft,
            OptionAsAlt::OnlyLeft => OptionAsAlt::OnlyRight,
            OptionAsAlt::OnlyRight => OptionAsAlt::Both,
            OptionAsAlt::Both => OptionAsAlt::None,
        };
        info!("Setting option as alt {:?}", self.option_as_alt);
        self.window.set_option_as_alt(self.option_as_alt);
    }

    /// Swap the window dimensions with `request_inner_size`.
    pub(crate) fn swap_dimensions(&mut self) {
        let old_inner_size = self.window.inner_size();
        let mut inner_size = old_inner_size;

        mem::swap(&mut inner_size.width, &mut inner_size.height);
        info!("Requesting resize from {old_inner_size:?} to {inner_size:?}");

        if let Some(new_inner_size) = self.window.request_inner_size(inner_size)
        {
            if old_inner_size == new_inner_size {
                info!("Inner size change got ignored");
            } else {
                self.resize(new_inner_size);
            }
        } else {
            info!("Request inner size is asynchronous");
        }
    }

    /// Pick the next cursor.
    pub(crate) fn next_cursor(&mut self) {
        self.named_idx = (self.named_idx + 1) % CURSORS.len();
        info!("Setting cursor to \"{:?}\"", CURSORS[self.named_idx]);
        self.window
            .set_cursor(Cursor::Icon(CURSORS[self.named_idx]));
    }

    /// Pick the next custom cursor.
    pub(crate) fn next_custom_cursor(
        &mut self,
        custom_cursors: &[CustomCursor],
    ) {
        self.custom_idx = (self.custom_idx + 1) % custom_cursors.len();
        let cursor = Cursor::Custom(custom_cursors[self.custom_idx].clone());
        self.window.set_cursor(cursor);
    }

    /// Custom cursor from an URL.
    #[cfg(web_platform)]
    pub(crate) fn url_custom_cursor(
        &mut self,
        event_loop: &ActiveEventLoop,
    ) -> Result<(), Box<dyn Error>> {
        // use std::error::Error;

        // use winit::event_loop::ActiveEventLoop;

        let cursor =
            event_loop.create_custom_cursor(self.url_custom_cursor())?;

        self.window.set_cursor(cursor);

        Ok(())
    }

    /// Custom cursor from a URL.
    #[cfg(web_platform)]
    pub(crate) fn animation_custom_cursor(
        &mut self,
        event_loop: &ActiveEventLoop,
        custom_cursors: &[CustomCursor],
    ) -> Result<(), Box<dyn Error>> {
        use std::time::Duration;

        let cursors = vec![
            custom_cursors[0].clone(),
            custom_cursors[1].clone(),
            event_loop.create_custom_cursor(url_custom_cursor())?,
        ];
        let cursor =
            CustomCursor::from_animation(Duration::from_secs(3), cursors)
                .unwrap();
        let cursor = event_loop.create_custom_cursor(cursor)?;

        self.window.set_cursor(cursor);

        Ok(())
    }

    /// Resize the window to the new size.
    pub(crate) fn resize(&mut self, size: PhysicalSize<u32>) {
        info!("Resized to {size:?}");
        #[cfg(not(any(android_platform, ios_platform)))]
        {
            // let (Some(width), Some(height)) =
            //     (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
            // else {
            //     return;
            // };
            // self.surface
            //     .resize(width, height)
            //     .expect("failed to resize inner buffer");
        }
        self.window.request_redraw();
    }

    /// Change the theme that things are drawn in.
    pub(crate) fn set_draw_theme(&mut self, theme: Theme) {
        self.theme = theme;
        self.window.request_redraw();
    }

    /// Show window menu.
    pub(crate) fn show_menu(&self) {
        if let Some(position) = self.cursor_position {
            self.window.show_window_menu(position);
        }
    }

    /// Drag the window.
    pub(crate) fn drag_window(&self) {
        if let Err(err) = self.window.drag_window() {
            info!("Error starting window drag: {err}");
        } else {
            info!("Dragging window Window={:?}", self.window.id());
        }
    }

    /// Drag-resize the window.
    pub(crate) fn drag_resize_window(&self) {
        let Some(position) = self.cursor_position else {
            info!("Drag-resize requires cursor to be inside the window");
            return;
        };

        let win_size = self.window.inner_size();
        let border_size = BORDER_SIZE * self.window.scale_factor();

        let x_direction = if position.x < border_size {
            ResizeDirection::West
        } else if position.x > (win_size.width as f64 - border_size) {
            ResizeDirection::East
        } else {
            // Use arbitrary direction instead of None for simplicity.
            ResizeDirection::SouthEast
        };

        let y_direction = if position.y < border_size {
            ResizeDirection::North
        } else if position.y > (win_size.height as f64 - border_size) {
            ResizeDirection::South
        } else {
            // Use arbitrary direction instead of None for simplicity.
            ResizeDirection::SouthEast
        };

        let direction = match (x_direction, y_direction) {
            (ResizeDirection::West, ResizeDirection::North) => {
                ResizeDirection::NorthWest
            }
            (ResizeDirection::West, ResizeDirection::South) => {
                ResizeDirection::SouthWest
            }
            (ResizeDirection::West, _) => ResizeDirection::West,
            (ResizeDirection::East, ResizeDirection::North) => {
                ResizeDirection::NorthEast
            }
            (ResizeDirection::East, ResizeDirection::South) => {
                ResizeDirection::SouthEast
            }
            (ResizeDirection::East, _) => ResizeDirection::East,
            (_, ResizeDirection::South) => ResizeDirection::South,
            (_, ResizeDirection::North) => ResizeDirection::North,
            _ => return,
        };

        if let Err(err) = self.window.drag_resize_window(direction) {
            info!("Error starting window drag-resize: {err}");
        } else {
            info!("Drag-resizing window Window={:?}", self.window.id());
        }
    }

    /// Change window occlusion state.
    pub(crate) fn set_occluded(&mut self, occluded: bool) {
        self.occluded = occluded;
        if !occluded {
            self.window.request_redraw();
        }
    }

    /// Draw the window contents.
    #[cfg(not(any(android_platform, ios_platform)))]
    pub(crate) fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        // const WHITE: u32 = 0xffff_ffff;
        // const DARK_GRAY: u32 = 0xff18_1818;

        // info!("WindowState::draw() entry");
        // if self.occluded {
        //     info!("Skipping drawing occluded window={:?}", self.window.id());
        //     return Ok(());
        // }

        // let color = match self.theme {
        //     Theme::Light => WHITE,
        //     Theme::Dark => DARK_GRAY,
        // };

        // let mut buffer = self.surface.buffer_mut()?;
        // buffer.fill(color);
        // self.window.pre_present_notify();
        // buffer.present()?;

        let frame = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: None },
        );

        let vertex_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Points"),
                    contents: bytemuck::cast_slice(&self.verticies),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        let index_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index buffer"),
                    contents: bytemuck::cast_slice(&self.indicies),
                    usage: wgpu::BufferUsages::INDEX,
                });

        {
            let mut rpass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: wgpu::StoreOp::Store,
                            },
                        },
                    )],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
            rpass.set_index_buffer(
                index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );
            // instances 0..1 implies instancing is not being used!!!.
            rpass.draw_indexed(0..self.indicies.len() as u32, 0, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }

    #[cfg(any(android_platform, ios_platform))]
    pub(crate) fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Drawing but without rendering...");
        Ok(())
    }
}
