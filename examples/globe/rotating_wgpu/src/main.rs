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

use ::tracing::info;
use winit::event::MouseButton;
use winit::event_loop::EventLoop;
use winit::keyboard::ModifiersState;
use winit::window::CustomCursor;
use winit::window::CustomCursorSource;
use winit::window::Icon;
use winit::window::Theme;

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
