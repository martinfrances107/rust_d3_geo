use winit::{event::MouseButton, keyboard::ModifiersState, window::Theme};

use crate::action::Action;

pub(crate) struct Binding<T: Eq> {
    pub(crate) trigger: T,
    pub(crate) mods: ModifiersState,
    pub(crate) action: Action,
}

impl<T: Eq> Binding<T> {
    pub(crate) const fn new(
        trigger: T,
        mods: ModifiersState,
        action: Action,
    ) -> Self {
        Self {
            trigger,
            mods,
            action,
        }
    }

    pub(crate) fn is_triggered_by(
        &self,
        trigger: &T,
        mods: ModifiersState,
    ) -> bool {
        &self.trigger == trigger && self.mods == mods
    }
}

pub(crate) const KEY_BINDINGS: &[Binding<&'static str>] = &[
    Binding::new("Q", ModifiersState::CONTROL, Action::CloseWindow),
    Binding::new("H", ModifiersState::CONTROL, Action::PrintHelp),
    Binding::new("F", ModifiersState::CONTROL, Action::ToggleFullscreen),
    Binding::new("D", ModifiersState::CONTROL, Action::ToggleDecorations),
    Binding::new("I", ModifiersState::CONTROL, Action::ToggleImeInput),
    Binding::new("L", ModifiersState::CONTROL, Action::CycleCursorGrab),
    Binding::new("P", ModifiersState::CONTROL, Action::ToggleResizeIncrements),
    Binding::new("R", ModifiersState::CONTROL, Action::ToggleResizable),
    Binding::new("R", ModifiersState::ALT, Action::RequestResize),
    // M.
    Binding::new("M", ModifiersState::CONTROL, Action::ToggleMaximize),
    Binding::new("M", ModifiersState::ALT, Action::Minimize),
    // N.
    Binding::new("N", ModifiersState::CONTROL, Action::CreateNewWindow),
    // C.
    Binding::new("C", ModifiersState::CONTROL, Action::NextCursor),
    Binding::new("C", ModifiersState::ALT, Action::NextCustomCursor),
    #[cfg(web_platform)]
    Binding::new(
        "C",
        ModifiersState::CONTROL.union(ModifiersState::SHIFT),
        Action::UrlCustomCursor,
    ),
    #[cfg(web_platform)]
    Binding::new(
        "C",
        ModifiersState::ALT.union(ModifiersState::SHIFT),
        Action::AnimationCustomCursor,
    ),
    Binding::new("Z", ModifiersState::CONTROL, Action::ToggleCursorVisibility),
    // K.
    Binding::new("K", ModifiersState::empty(), Action::SetTheme(None)),
    Binding::new(
        "K",
        ModifiersState::SUPER,
        Action::SetTheme(Some(Theme::Light)),
    ),
    Binding::new(
        "K",
        ModifiersState::CONTROL,
        Action::SetTheme(Some(Theme::Dark)),
    ),
    #[cfg(macos_platform)]
    Binding::new("T", ModifiersState::SUPER, Action::CreateNewTab),
    #[cfg(macos_platform)]
    Binding::new("O", ModifiersState::CONTROL, Action::CycleOptionAsAlt),
];

pub(crate) const MOUSE_BINDINGS: &[Binding<MouseButton>] = &[
    Binding::new(
        MouseButton::Left,
        ModifiersState::ALT,
        Action::DragResizeWindow,
    ),
    Binding::new(
        MouseButton::Left,
        ModifiersState::CONTROL,
        Action::DragWindow,
    ),
    Binding::new(
        MouseButton::Right,
        ModifiersState::CONTROL,
        Action::ShowWindowMenu,
    ),
];
