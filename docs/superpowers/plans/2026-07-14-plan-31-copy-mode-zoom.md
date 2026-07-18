# Plan 31: Copy Mode + Pane Zoom

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans.

**Goal:** Implement copy mode (vi-style scrollback browsing + text selection + copy to clipboard) and pane zoom (maximize pane to full window).

**Dependencies:** `terminal_view`, `mux`, `clipboard`.

**Spec:** §12 Terminal Product Details (post-foundation)

---

### Task 1: Copy mode

**Files:**
- Create: `crates/terminal_view/src/copy_mode.rs`

- [ ] **Step 1: Implement vi-style navigation**

hjkl 移动光标，gg/G 跳转，/ 搜索，q 退出。

- [ ] **Step 2: Implement text selection**

v 进入 visual 模式，V 行选择，y 复制到剪贴板。

- [ ] **Step 3: Integrate with input routing**

Copy mode 激活时拦截所有按键（已在 Plan 21 中预留）。

---

### Task 2: Pane zoom

**Files:**
- Modify: `crates/workspace/src/pane_group.rs`

- [ ] **Step 1: Implement zoom toggle**

Zoom 时隐藏其他 pane，当前 pane 占据全部空间。

- [ ] **Step 2: Keymap binding**

`ctrl-shift-z` / prefix+z 触发 zoom toggle。

---

### Task 3: Tests + Commit

- [ ] `cargo check` passes
- [ ] Commit + push
