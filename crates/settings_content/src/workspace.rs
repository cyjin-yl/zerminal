use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

/// UI chrome workspace settings (spec §16 Plan 16)
#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct WorkspaceSettingsContent {
    /// What draws window decorations/titlebar. Default: client
    pub window_decorations: WindowDecorations,

    /// The text rendering mode to use. Default: platform_default
    pub text_rendering_mode: TextRenderingMode,

    /// Whether the focused panel follows the mouse location.
    pub focus_follows_mouse: FocusFollowsMouse,

    /// Whether or not to prompt the user to confirm before closing the application. Default: false
    pub confirm_quit: bool,

    /// What to do when the last window is closed.
    pub on_last_window_closed: OnLastWindowClosed,
}

/// What draws window decorations/titlebar.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum WindowDecorations {
    /// Use system-provided window decorations.
    System,
    /// Use client-provided window decorations.
    #[default]
    Client,
    /// No window decorations.
    None,
}

/// The text rendering mode to use.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum TextRenderingMode {
    /// Use the platform default.
    #[default]
    PlatformDefault,
    /// Use software rendering.
    Software,
    /// Use anti-aliased rendering.
    AntiAliased,
}

/// Whether the focused panel follows the mouse location.
#[with_fallible_options]
#[derive(Copy, Clone, PartialEq, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct FocusFollowsMouse {
    /// Whether focus follows the mouse. Default: false
    pub enabled: bool,
}

/// What to do when the last window is closed.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum OnLastWindowClosed {
    /// Do nothing.
    #[default]
    Nothing,
    /// Quit the application.
    Quit,
}

/// Tab settings for terminal panes (spec §16 Plan 16)
#[with_fallible_options]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct TabBarSettingsContent {
    /// Whether to show the middle click to close tab behavior. Default: true
    pub middle_click_to_close: bool,

    /// Whether to show the mouse scroll to switch tab behavior. Default: true
    pub mouse_scroll_to_switch: bool,

    /// Whether to show the active item only. Default: false
    pub show_active_item: bool,

    /// Whether to show the button to close a tab. Default: hover
    pub show_close_button: ShowCloseButton,
}

/// Position of the close button in a tab.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "lowercase")]
pub enum ShowCloseButton {
    /// Show when the mouse hovers over the tab.
    #[default]
    Hover,
    /// Always show.
    Always,
    /// Never show.
    Never,
}

/// Status bar settings (spec §16 Plan 16)
#[with_fallible_options]
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug, PartialEq, Eq)]
pub struct StatusBarSettingsContent {
    /// Whether to show the stack size on the status bar. Default: false
    pub stack_size: bool,

    /// Whether to show the working directory on the status bar. Default: true
    pub working_directory: bool,

    /// Whether to show the session status on the status bar. Default: false
    pub session_status: bool,
}
