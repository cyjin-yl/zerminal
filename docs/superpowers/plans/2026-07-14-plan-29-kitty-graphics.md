# Plan 29: Kitty Graphics Protocol / iTerm2 OSC 1337

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans.

**Goal:** Implement Kitty graphics protocol (kitty-graphics) and iTerm2 OSC 1337 image display support. Sixel is already supported by alacritty. These are enhancements for displaying images in terminal panes.

**Dependencies:** `terminal` (alacritty-based), `mux_server`.

**Spec:** §11.2 Terminal Emulation (post-foundation)

---

### Task 1: Kitty graphics protocol parser

**Files:**
- Create: `crates/terminal/src/kitty_graphics.rs`

- [ ] **Step 1: Implement OSC 1337 parser (iTerm2)**

Parse `ESC ] 1337 ; File=name=...;inline=1 : base64_data ST`

- [ ] **Step 2: Implement kitty-graphics protocol parser**

Parse `ESC _ G f=100,... ; base64_data ESC \ `

- [ ] **Step 3: Image cache management**

Cache decoded images per-pane, with size limits.

---

### Task 2: GPUI image rendering

**Files:**
- Modify: `crates/terminal_view/src/terminal_view.rs`

- [ ] **Step 1: Render images in terminal grid**

When image cells are present, render using GPUI Image element instead of text.

- [ ] **Step 2: Image cell positioning**

Images span multiple cells (width/height in cells).

---

### Task 3: Tests + Commit

- [ ] `cargo check` passes
- [ ] Commit + push
