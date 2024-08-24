use core::mem;
use std::error::Error;
use std::sync::Arc;

#[cfg(not(any(android_platform, ios_platform)))]
use std::num::NonZeroU32;
use wgpu::rwh::DisplayHandle;
use winit::{
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize},
    keyboard::ModifiersState,
    window::{
        Cursor, CursorGrabMode, CustomCursor, Fullscreen, ResizeDirection,
        Theme, Window,
    },
};

#[cfg(not(any(android_platform, ios_platform)))]
use softbuffer::Surface;

use ::tracing::{error, info};

use crate::{app::Application, BORDER_SIZE, CURSORS};

/// State of the window.
pub(crate) struct WindowState {
    /// IME input.
    ime: bool,

    /// Render surface.
    ///
    /// NOTE: This surface must be dropped before the `Window`.
    #[cfg(not(any(android_platform, ios_platform)))]
    surface: Surface<DisplayHandle<'static>, Arc<Window>>,

    /// The actual winit Window.
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

impl WindowState {
    pub(crate) fn new(
        app: &Application,
        window: Window,
    ) -> Result<Self, Box<dyn Error>> {
        let window = Arc::new(window);

        // SAFETY: the surface is dropped before the `window` which provided it with handle, thus
        // it doesn't outlive it.
        #[cfg(not(any(android_platform, ios_platform)))]
        let surface =
            Surface::new(app.context.as_ref().unwrap(), Arc::clone(&window))?;

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
            let (Some(width), Some(height)) =
                (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
            else {
                return;
            };
            self.surface
                .resize(width, height)
                .expect("failed to resize inner buffer");
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
        const WHITE: u32 = 0xffff_ffff;
        const DARK_GRAY: u32 = 0xff18_1818;

        info!("WindowState::draw() entry");
        if self.occluded {
            info!("Skipping drawing occluded window={:?}", self.window.id());
            return Ok(());
        }

        let color = match self.theme {
            Theme::Light => WHITE,
            Theme::Dark => DARK_GRAY,
        };

        let mut buffer = self.surface.buffer_mut()?;
        buffer.fill(color);
        self.window.pre_present_notify();
        buffer.present()?;
        Ok(())
    }

    #[cfg(any(android_platform, ios_platform))]
    pub(crate) fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Drawing but without rendering...");
        Ok(())
    }
}
