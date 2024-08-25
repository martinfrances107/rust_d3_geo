use core::fmt::{self, Debug};

use winit::window::Theme;

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
    pub(crate) const fn help(self) -> &'static str {
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
