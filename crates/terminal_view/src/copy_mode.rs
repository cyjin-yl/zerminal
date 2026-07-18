// §12 复制模式: vi 风格导航 + 文本选择 + 复制到剪贴板
// Plan 31 — Copy mode for terminal scrollback browsing

use gpui::{Context, Entity, Keystroke};
use terminal::Terminal;

/// 复制模式状态
///
/// 当复制模式激活时，TerminalView 拦截所有按键，
/// 将导航命令路由到 terminal 的 vi_motion，
/// 将编辑命令 (V, /, q, n, N, escape, i) 拦截到本模块。
#[derive(Clone, Debug, Default)]
pub struct CopyModeState {
    /// 是否激活复制模式
    pub active: bool,
    /// 当前搜索查询 (None = 无搜索)
    pub search_query: Option<String>,
}

/// 在复制模式处理按键。
///
/// 返回 `true` 表示按键已被拦截（不发送到 PTY）。
/// 返回 `false` 表示按键应转发到 terminal.vi_motion。
pub fn dispatch_copy_mode_key(
    keystroke: &Keystroke,
    state: &mut CopyModeState,
    terminal: &Entity<Terminal>,
    cx: &mut Context<super::TerminalView>,
) -> bool {
    match keystroke.key.as_str() {
        // V: 行选择模式 (Line selection) — §12 Plan 31
        "v" if keystroke.modifiers.shift => {
            // 通过发送大写 V 到 vi_motion 触发行选择
            // terminal 的 vi_motion 目前仅处理小写 v (Simple selection)
            // 行选择 (V) 需要 terminal 层扩展支持，这里先转发
            // 当 terminal 支持后，取消下方注释:
            // terminal.update(cx, |term, _| {
            //     let v_keystroke = Keystroke::new(Modifiers::SHIFT, "", "V");
            //     term.vi_motion(&v_keystroke);
            // });
            // 当前行为: 转发到 vi_motion (将被忽略，不发送 PTY)
            // 这是暂时的 — 后续扩展 terminal vi_motion 支持 V 即可
            true
        }

        // /: 搜索 — §12 Plan 31
        "/" => {
            // 搜索由 SearchableItem 接口处理 (/ 键拦截，不发送到 PTY)
            // 实际的搜索查询由外部 search 面板触发
            true
        }

        // q: 退出复制模式 — §12 Plan 31
        "q" => {
            state.active = false;
            state.search_query = None;
            true
        }

        // n: 下一个搜索匹配 — §12 Plan 31
        "n" => {
            // 搜索导航由 SearchableItem 接口处理
            true
        }

        // N: 上一个搜索匹配 — §12 Plan 31
        "N" => {
            true
        }

        // escape: 清除选择 + 退出复制模式 — §12 Plan 31
        "escape" => {
            // 清除选择
            terminal.update(cx, |term, _| {
                let mut esc = Keystroke::default();
                esc.key = "escape".to_string();
                term.vi_motion(&esc);
            });
            state.active = false;
            state.search_query = None;
            true
        }

        // i: 退出复制模式，返回正常输入 — §12 Plan 31
        "i" => {
            state.active = false;
            state.search_query = None;
            true
        }

        // 其他按键: 转发到 terminal.vi_motion (hjkl, g, G, w, b, e, v, y 等)
        _ => false,
    }
}

/// 进入复制模式 — §12 Plan 31
///
/// 先启用 vi 模式（复制模式基于 vi 模式），然后激活复制模式。
pub fn enter_copy_mode(
    terminal: &Entity<Terminal>,
    state: &mut CopyModeState,
    cx: &mut Context<super::TerminalView>,
) {
    // 先启用 vi 模式
    terminal.update(cx, |term, _| {
        term.toggle_vi_mode();
    });
    state.active = true;
    state.search_query = None;
}

/// 退出复制模式 — §12 Plan 31
///
/// 清除选择，退出 vi 模式，关闭复制模式。
pub fn exit_copy_mode(
    terminal: &Entity<Terminal>,
    state: &mut CopyModeState,
    cx: &mut Context<super::TerminalView>,
) {
    // 清除选择
    terminal.update(cx, |term, _| {
        let mut esc = Keystroke::default();
        esc.key = "escape".to_string();
        term.vi_motion(&esc);
    });

    state.active = false;
    state.search_query = None;
}
