mod action;
mod extension;
mod fallible_options;
mod mux;
pub mod merge_from;
mod project;
mod serde_helper;
mod shadow_snapshot;
mod terminal;
mod theme;
mod title_bar;
mod workspace;

pub use action::{ActionName, ActionWithArguments, CommandAliasTarget};
pub use extension::*;
pub use fallible_options::*;
pub use merge_from::MergeFrom as MergeFromTrait;
pub use mux::*;
pub use project::*;
use serde::de::DeserializeOwned;
pub use serde_helper::{
    serialize_f32_with_two_decimal_places, serialize_optional_f32_with_two_decimal_places,
};
use settings_json::parse_json_with_comments;
pub use shadow_snapshot::*;
pub use terminal::*;
pub use theme::*;
pub use title_bar::*;
pub use workspace::*;

use collections::{HashMap, IndexMap};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

/// 定义设置覆盖结构体，每个字段为 `Option<Box<SettingsContent>>`，
/// 同时生成 `OVERRIDE_KEYS` 和 `get_by_key` 方法。
macro_rules! settings_overrides {
    (
        $(#[$attr:meta])*
        pub struct $name:ident { $($field:ident),* $(,)? }
    ) => {
        $(#[$attr])*
        pub struct $name {
            $(pub $field: Option<Box<SettingsContent>>,)*
        }

        impl $name {
            /// JSON 覆盖键名，从此结构体的字段名派生。
            pub const OVERRIDE_KEYS: &[&str] = &[$(stringify!($field)),*];

            /// 通过 JSON 键名查找覆盖设置。
            pub fn get_by_key(&self, key: &str) -> Option<&SettingsContent> {
                match key {
                    $(stringify!($field) => self.$field.as_deref(),)*
                    _ => None,
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseStatus {
    /// 设置解析成功
    Success,
    /// 设置文件未变更，跳过解析
    Unchanged,
    /// 设置解析失败
    Failed { error: String },
}

/// 键盘输入时隐藏鼠标的时机 (spec §16 Plan 16)
/// 默认: on_typing_and_action
#[derive(
    Copy, Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema, MergeFrom,
    strum::VariantArray, strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum HideMouseMode {
    /// 不隐藏鼠标
    Never,
    /// 仅在打字时隐藏
    OnTyping,
    /// 打字和执行操作时隐藏
    #[default]
    OnTypingAndAction,
}

/// 终端/多路复用器/UI chrome 设置结构体 (spec §16 Plan 16)
#[with_fallible_options]
#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct SettingsContent {
    #[serde(flatten)]
    pub project: ProjectSettingsContent,

    #[serde(flatten)]
    pub theme: Box<ThemeSettingsContent>,

    #[serde(flatten)]
    pub extension: ExtensionSettingsContent,

    #[serde(flatten)]
    pub workspace: WorkspaceSettingsContent,

    /// 远程连接设置 (spec §16 Plan 16)
    #[serde(flatten)]
    pub remote: RemoteSettingsContent,

    /// 终端设置 (spec §16 Plan 16)
    pub terminal: Option<TerminalSettingsContent>,

    /// 多路复用器设置 (spec §16 Plan 16)
    pub mux: Option<MuxSettingsContent>,

    /// 影子快照设置 (spec §16 Plan 16)
    pub shadow_snapshot: Option<ShadowSnapshotSettingsContent>,

    /// 标题栏设置 (spec §16 Plan 16)
    pub title_bar: Option<TitleBarSettingsContent>,

    /// Tab 栏设置 (spec §16 Plan 16)
    pub tab_bar: Option<TabBarSettingsContent>,

    /// 状态栏设置 (spec §16 Plan 16)
    pub status_bar: Option<StatusBarSettingsContent>,

    /// 基础键盘映射方案
    /// 默认: VSCode
    pub base_keymap: Option<BaseKeymapContent>,

    /// 鼠标隐藏模式
    pub hide_mouse: Option<HideMouseMode>,

    /// 自动更新
    pub auto_update: Option<bool>,

    /// 遥测设置
    pub telemetry: Option<TelemetrySettingsContent>,

    /// 日志范围到级别的映射
    pub log: Option<HashMap<String, String>>,

    /// 功能标志本地覆盖
    pub feature_flags: Option<FeatureFlagsMap>,
}

/// 远程连接设置 (spec §16 Plan 16)
#[with_fallible_options]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
pub struct RemoteSettingsContent {
    /// 远程服务器路径
    pub remote_server_path: Option<String>,

    /// 是否自动安装远程服务器。默认: true
    pub auto_install: bool,
}

/// 工具遥测设置
#[with_fallible_options]
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Debug, MergeFrom)]
pub struct TelemetrySettingsContent {
    /// 是否收集诊断事件
    pub diagnostics: bool,
    /// 是否收集应用事件
    pub events: bool,
    /// 是否收集 metrics
    pub metrics: bool,
}

impl Default for TelemetrySettingsContent {
    fn default() -> Self {
        Self {
            diagnostics: true,
            events: true,
            metrics: true,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, MergeFrom)]
#[serde(transparent)]
pub struct FeatureFlagsMap(pub HashMap<String, String>);

impl JsonSchema for FeatureFlagsMap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "FeatureFlagsMap".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "object",
            "additionalProperties": { "type": "string" }
        })
    }
}

impl std::ops::Deref for FeatureFlagsMap {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for FeatureFlagsMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// 优化构建避免下游单态化
pub trait RootUserSettings: Sized + DeserializeOwned {
    fn parse_json(json: &str) -> (Option<Self>, ParseStatus);
    fn parse_json_with_comments(json: &str) -> anyhow::Result<Self>;
}

impl RootUserSettings for SettingsContent {
    fn parse_json(json: &str) -> (Option<Self>, ParseStatus) {
        fallible_options::parse_json(json)
    }
    fn parse_json_with_comments(json: &str) -> anyhow::Result<Self> {
        parse_json_with_comments(json)
    }
}

impl RootUserSettings for Option<SettingsContent> {
    fn parse_json(json: &str) -> (Option<Self>, ParseStatus) {
        fallible_options::parse_json(json)
    }
    fn parse_json_with_comments(json: &str) -> anyhow::Result<Self> {
        parse_json_with_comments(json)
    }
}

impl RootUserSettings for UserSettingsContent {
    fn parse_json(json: &str) -> (Option<Self>, ParseStatus) {
        fallible_options::parse_json(json)
    }
    fn parse_json_with_comments(json: &str) -> anyhow::Result<Self> {
        parse_json_with_comments(json)
    }
}

settings_overrides! {
    #[with_fallible_options]
    #[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
    pub struct ReleaseChannelOverrides { dev, nightly, preview, stable }
}

settings_overrides! {
    #[with_fallible_options]
    #[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
    pub struct PlatformOverrides { macos, linux, windows }
}

/// 配置文件基于的基础设置
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum ProfileBase {
    /// 在用户设置之上应用配置文件覆盖
    #[default]
    User,
    /// 在默认设置之上应用配置文件覆盖，忽略用户自定义
    Default,
}

/// 命名配置文件，可以临时覆盖设置
#[with_fallible_options]
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct SettingsProfile {
    /// 应用此配置文件覆盖之前的基础设置
    #[serde(default)]
    pub base: ProfileBase,

    /// 此配置文件的设置覆盖
    #[serde(default)]
    pub settings: Box<SettingsContent>,
}

#[with_fallible_options]
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct UserSettingsContent {
    #[serde(flatten)]
    pub content: Box<SettingsContent>,

    #[serde(flatten)]
    pub release_channel_overrides: ReleaseChannelOverrides,

    #[serde(flatten)]
    pub platform_overrides: PlatformOverrides,

    #[serde(default)]
    pub profiles: IndexMap<String, SettingsProfile>,
}

/// 基础键盘映射方案
#[derive(
    Copy, Clone, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq,
    strum::VariantArray, strum::VariantNames, strum::FromRepr,
)]
#[serde(rename_all = "snake_case")]
pub enum BaseKeymapContent {
    /// VSCode 键盘映射
    #[default]
    VSCode,
    /// JetBrains 键盘映射
    JetBrains,
    /// Sublime Text 键盘映射
    SublimeText,
    /// Vim 键盘映射
    Vim,
    /// Zed 默认键盘映射
    Zed,
    /// Helix 键盘映射
    Helix,
    /// Atom 键盘映射
    Atom,
    /// TextMate 键盘映射
    TextMate,
    /// Emacs 键盘映射
    Emacs,
    /// Cursor 键盘映射
    Cursor,
    /// 无键盘映射
    None,
}

/// 兼容占位类型: SaturatingBool (spec §16 Plan 16)
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SaturatingBool(pub bool);

impl std::fmt::Display for SaturatingBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<bool> for SaturatingBool {
    fn from(b: bool) -> Self { Self(b) }
}

impl From<SaturatingBool> for bool {
    fn from(s: SaturatingBool) -> Self { s.0 }
}

impl merge_from::MergeFrom for SaturatingBool {
    fn merge_from(&mut self, other: &Self) {
        self.0 = self.0 || other.0;
    }
}
// 以下为已删除模块的兼容占位类型 (spec §16 Plan 16)
// 保留以兼容 settings_store 中尚未清理的引用。

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct SemanticTokenRules {
    pub rules: Vec<SemanticTokenRule>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct SemanticTokenRule {
    pub token_type: Option<String>,
    pub token_modifiers: Vec<String>,
    pub style: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct ExtensionsSettingsContent {
    pub all_languages: LanguageToSettingsMap,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct LanguageToSettingsMap {
    pub settings: HashMap<String, LanguageSettingsContent>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct LanguageSettingsContent {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct LspSettings {}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct LspSettingsMap {
    pub settings: HashMap<String, LspSettings>,
}
