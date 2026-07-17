use collections::HashMap;
use extension::{
    DownloadFileCapability, ExtensionCapability, NpmInstallPackageCapability, ProcessExecCapability,
};
use settings::Settings;
use std::sync::Arc;

/// 扩展设置 (spec §16 Plan 16)
/// 原 auto_install_extensions, auto_update_extensions, granted_capabilities 已移除
#[derive(Debug, Default, Clone)]
pub struct ExtensionSettings {
    /// 自动安装的扩展 (原 settings 已移除, 使用空默认值)
    pub auto_install_extensions: HashMap<Arc<str>, bool>,
    /// 自动更新的扩展 (原 settings 已移除, 使用空默认值)
    pub auto_update_extensions: HashMap<Arc<str>, bool>,
    /// 已授予的能力 (原 settings 已移除, 使用空默认值)
    pub granted_capabilities: Vec<ExtensionCapability>,
}

impl ExtensionSettings {
    /// 判断是否应该自动安装指定扩展
    pub fn should_auto_install(&self, extension_id: &str) -> bool {
        self.auto_install_extensions
            .get(extension_id)
            .copied()
            .unwrap_or(true)
    }

    /// 判断是否应该自动更新指定扩展
    pub fn should_auto_update(&self, extension_id: &str) -> bool {
        self.auto_update_extensions
            .get(extension_id)
            .copied()
            .unwrap_or(true)
    }
}

impl Settings for ExtensionSettings {
    fn from_settings(_content: &settings::SettingsContent) -> Self {
        // 原设置字段 (auto_install_extensions, auto_update_extensions, granted_extension_capabilities)
        // 已在 settings 重构中移除 (spec §16 Plan 16), 返回空默认值
        Self::default()
    }
}
