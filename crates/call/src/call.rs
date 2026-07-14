pub mod call_settings;

mod call_impl;

pub use call_impl::*;
// 来源: spec §8.2 Pass 1 — call crate 在 Plan 4 中被删除，临时恢复并标记为迁移洞

use zerminal_macros::zerminal_todo;

#[zerminal_todo("removed-crate", "call crate 已被删除，等待恢复")]
pub struct __ZerminalTodoMarker;
