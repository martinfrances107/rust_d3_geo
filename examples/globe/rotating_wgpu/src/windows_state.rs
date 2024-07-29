use tracing::{error, info};
use wgpu::{rwh::DisplayHandle};
use winit::{dpi::PhysicalPosition, keyboard::ModifiersState, window::{CursorGrabMode, Theme, Window}};

use std::{borrow::Cow, error::Error, sync::Arc};
use env_logger::builder;
use wgpu::{Adapter, Device, PipelineLayout, RenderPipeline, ShaderModule, Surface, SurfaceConfiguration};
use winit::{
    application::ApplicationHandler,  event::{Event, WindowEvent}, event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
};
use winit::window::{ WindowId};

use crate::app::App;
use crate::cursors::CURSORS;

/// State of the window.
pub(crate) struct WindowState<'a> {
  /// IME input.
  ime: bool,
  /// Render surface.
  ///
  /// NOTE: This surface must be dropped before the `Window`.
  // #[cfg(not(any(android_platform, ios_platform)))]
  // surface: Surface<DisplayHandle<'static>, Arc<Window>>,
  //
  // MAYBE BUG
  // I am mixing code samples Surface here is from softbuffer or from WGPU
  // depending of the example I copiying from?????
  //
  surface: Surface<'a>,
  /// The actual winit Window.
  window: Arc<Window>,
  /// The window theme we're drawing with.
  theme: Theme,
  /// Cursor position over the window.
  cursor_position: Option<PhysicalPosition<f64>>,
  /// Window modifiers state.
  modifiers: ModifiersState,
  /// Occlusion state of the window.
  occluded: bool,
  /// Current cursor grab mode.
  cursor_grab: CursorGrabMode,
  /// The amount of zoom into window.
  zoom: f64,
  /// The amount of rotation of the window.
  rotated: f32,
  /// The amount of pan of the window.
  panned: PhysicalPosition<f32>,

  #[cfg(macos_platform)]
  option_as_alt: OptionAsAlt,

  // Cursor states.
  named_idx: usize,
  custom_idx: usize,
  cursor_hidden: bool,
}

impl <'a> WindowState<'a> {
  pub(crate) fn new(app: &App, window: Window) -> Result<Self, Box<dyn Error>> {
      let window = Arc::new(window);

      // SAFETY: the surface is dropped before the `window` which provided it with handle, thus
      // it doesn't outlive it.
      #[cfg(not(any(android_platform, ios_platform)))]
      let surface = Surface::new(app.context.as_ref().unwrap(), Arc::clone(&window))?;

      let theme = window.theme().unwrap_or(Theme::Dark);
      info!("Theme: {theme:?}");
      let named_idx = 0;
      window.set_cursor(CURSORS[named_idx]);

      // Allow IME out of the box.
      let ime = true;
      window.set_ime_allowed(ime);

      let size = window.inner_size();
      let mut state = Self {
          #[cfg(macos_platform)]
          option_as_alt: window.option_as_alt(),
          custom_idx: app.custom_cursors.len() - 1,
          cursor_grab: CursorGrabMode::None,
          named_idx,
          #[cfg(not(any(android_platform, ios_platform)))]
          surface,
          window,
          theme,
          ime,
          cursor_position: Default::default(),
          cursor_hidden: Default::default(),
          modifiers: Default::default(),
          occluded: Default::default(),
          rotated: Default::default(),
          panned: Default::default(),
          zoom: Default::default(),
      };

      state.resize(size);
      Ok(state)
  }
}