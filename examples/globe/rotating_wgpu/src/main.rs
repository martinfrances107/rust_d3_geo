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

pub(crate) mod action;
pub(crate) mod app;
pub(crate) mod bindings;
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
    let event_loop_proxy = event_loop.create_proxy();

    // Wire the user event from another thread.
    #[cfg(not(web_platform))]
    std::thread::spawn(move || {
        // Wake up the `event_loop` once every second and dispatch a custom event
        // from a different thread.
        info!("Starting to send user event every second");
        loop {
            let _ = event_loop_proxy.send_event(UserEvent::WakeUp);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    let mut state = Application::new(&event_loop);

    event_loop.run_app(&mut state).map_err(Into::into)
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
