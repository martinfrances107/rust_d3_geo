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

use std::borrow::Cow;

use geo_types::Coord;
use geo_types::Geometry;
use wgpu::util::DeviceExt;
use wgpu::IndexFormat;
use wgpu::PipelineCompilationOptions;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::window::Window;

use d3_geo_rs::graticule::generate_mls;
use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::path::wgpu::polylines::PolyLines as PolyLinesWGPU;
use d3_geo_rs::path::wgpu::Vertex;
use d3_geo_rs::projection::orthographic::Orthographic;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::RawBase;
use d3_geo_rs::projection::ScaleSet;
use d3_geo_rs::projection::TranslateSet;

async fn run(event_loop: EventLoop<()>, window: Window) {
    let mut size = window.inner_size();
    size.width = size.width.max(1);
    size.height = size.height.max(1);

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

    let mut config = surface
        .get_default_config(&adapter, size.width, size.height)
        .unwrap();
    surface.configure(&device, &config);

    let window = &window;
    event_loop
        .run(move |event, target| {
            // Have the closure take ownership of the resources.
            // `event_loop.run` never returns, therefore we must do this to ensure
            // the resources are properly cleaned up.
            let _ = (&instance, &adapter, &shader, &pipeline_layout);

            if let Event::WindowEvent {
                window_id: _,
                event,
            } = event
            {
                match event {
                    WindowEvent::Resized(new_size) => {
                        // Reconfigure the surface with the new size
                        config.width = new_size.width.max(1);
                        config.height = new_size.height.max(1);
                        surface.configure(&device, &config);
                        // On macos the window needs to be redrawn manually after resizing
                        window.request_redraw();
                    }
                    WindowEvent::RedrawRequested => {
                        let frame = surface.get_current_texture().expect(
                            "Failed to acquire next swap chain texture",
                        );
                        let view = frame.texture.create_view(
                            &wgpu::TextureViewDescriptor::default(),
                        );
                        let mut encoder = device.create_command_encoder(
                            &wgpu::CommandEncoderDescriptor { label: None },
                        );

                        let vertex_buffer = device.create_buffer_init(
                            &wgpu::util::BufferInitDescriptor {
                                label: Some("Points"),
                                contents: bytemuck::cast_slice(&verticies),
                                usage: wgpu::BufferUsages::VERTEX,
                            },
                        );

                        let index_buffer = device.create_buffer_init(
                            &wgpu::util::BufferInitDescriptor {
                                label: Some("Index buffer"),
                                contents: bytemuck::cast_slice(&indicies),
                                usage: wgpu::BufferUsages::INDEX,
                            },
                        );

                        {
                            let mut rpass = encoder.begin_render_pass(
                                &wgpu::RenderPassDescriptor {
                                    label: None,
                                    color_attachments: &[Some(
                                        wgpu::RenderPassColorAttachment {
                                            view: &view,
                                            resolve_target: None,
                                            ops: wgpu::Operations {
                                                load: wgpu::LoadOp::Clear(
                                                    wgpu::Color::BLACK,
                                                ),
                                                store: wgpu::StoreOp::Store,
                                            },
                                        },
                                    )],
                                    depth_stencil_attachment: None,
                                    timestamp_writes: None,
                                    occlusion_query_set: None,
                                },
                            );
                            rpass.set_pipeline(&render_pipeline);
                            rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
                            rpass.set_index_buffer(
                                index_buffer.slice(..),
                                wgpu::IndexFormat::Uint32,
                            );
                            // instances 0..1 implies instancing is not being used!!!.
                            rpass.draw_indexed(
                                0..indicies.len() as u32,
                                0,
                                0..1,
                            );
                        }

                        queue.submit(Some(encoder.finish()));
                        frame.present();
                    }
                    WindowEvent::CloseRequested => target.exit(),
                    _ => {}
                };
            }
        })
        .unwrap();
}

/// Entry point
///
/// Initialisation before the APP starts.
///
/// Logging
///
/// Environment sensing
///  eg. check build for HTML canvas if required.
pub fn main() {
    let event_loop = EventLoop::new().unwrap();
    #[allow(unused_mut)]
    let mut builder = winit::window::WindowBuilder::new();
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowBuilderExtWebSys;
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        builder = builder.with_canvas(Some(canvas));
    }

    let window = builder.build(&event_loop).unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(run(event_loop, window));
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        wasm_bindgen_futures::spawn_local(run(event_loop, window));
    }
}
