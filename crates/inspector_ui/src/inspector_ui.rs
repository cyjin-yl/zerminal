#[cfg(debug_assertions)]
mod div_inspector;
#[cfg(debug_assertions)]
mod inspector;

#[cfg(debug_assertions)]
pub use inspector::init;

#[cfg(not(debug_assertions))]
pub fn init(_app_state: std::sync::Arc<workspace::AppState>, cx: &mut gpui::App) {
    use std::any::TypeId;
    use workspace::notifications::NotifyResultExt as _;

    cx.on_action(|_: &zed_actions::dev::ToggleInspector, cx| {
        Err::<(), anyhow::Error>(anyhow::anyhow!(
            "dev::ToggleInspector is only available in debug builds"
        ))
        .notify_app_err(cx);
    });

    command_palette_hooks::CommandPaletteFilter::update_global(cx, |filter, _cx| {
        filter.hide_action_types(&[TypeId::of::<zed_actions::dev::ToggleInspector>()]);
    });
}
// 来源: spec §8.2 Pass 1 — inspector_ui crate 在 Plan 4 中被删除，临时恢复并标记为迁移洞

use zerminal_macros::zerminal_todo;

#[zerminal_todo("removed-crate", "inspector_ui crate 已被删除，等待恢复")]
pub struct __ZerminalTodoMarker;
