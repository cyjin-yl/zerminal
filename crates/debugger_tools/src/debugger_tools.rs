mod dap_log;
pub use dap_log::*;

use gpui::App;

pub fn init(cx: &mut App) {
    dap_log::init(cx);
}
// 来源: spec §8.2 Pass 1 — debugger_tools crate 在 Plan 4 中被删除，临时恢复并标记为迁移洞

use zerminal_macros::zerminal_todo;

#[zerminal_todo("removed-crate", "debugger_tools crate 已被删除，等待恢复")]
pub struct __ZerminalTodoMarker;
