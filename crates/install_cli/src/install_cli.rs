#[cfg(not(target_os = "windows"))]
mod install_cli_binary;
mod register_zed_scheme;

#[cfg(not(target_os = "windows"))]
pub use install_cli_binary::{InstallCliBinary, install_cli_binary};
pub use register_zed_scheme::{RegisterZedScheme, register_zed_scheme};
// 来源: spec §8.2 Pass 1 — install_cli crate 在 Plan 4 中被删除，临时恢复并标记为迁移洞

use zerminal_macros::zerminal_todo;

#[zerminal_todo("removed-crate", "install_cli crate 已被删除，等待恢复")]
pub struct __ZerminalTodoMarker;
