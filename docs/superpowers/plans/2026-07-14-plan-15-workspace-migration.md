# Plan 15: Workspace Migration — pane_group → server

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans.

**Goal:** Migrate all layout calculation logic from client-side `workspace/pane_group` to server-side `mux_server/layout`. Client workspace becomes a stateless layout renderer + GPUI item container.

**Architecture:** Server owns authoritative layout tree (split directions, ratios, resize math). Client receives `SessionSnapshot` / `SessionLayoutChanged` notifications and renders panes at server-specified positions. User interactions (drag divider, resize) are forwarded as RPCs.

**Dependencies:** `mux`, `mux_server`, `workspace`, `gpui`.

---

### Task 1: Migrate pane_group geometry to mux_server

**Files:**
- Modify: `crates/mux_server/src/layout.rs` (from Plan 10 Task 4)
- Source: `crates/workspace/src/pane_group.rs`

- [ ] **Step 1: Extract split tree data structure**

Copy the core `LayoutNode` tree structure (SplitDirection, children, ratios) from `pane_group.rs` into `mux_server/src/layout.rs`. Adapt types to use mux_protocol types.

- [ ] **Step 2: Extract resize math**

Copy the proportional resize algorithm from pane_group. When a divider is dragged, recompute child ratios.

- [ ] **Step 3: Implement layout operations**

- split(pane_id, direction) → creates new pane, adds split node
- close(pane_id) → removes pane, collapses parent split if needed
- resize(pane_id, cols, rows) → adjusts ratios
- focus(pane_id) → updates focused pane

Each operation mutates the authoritative tree and triggers `SessionLayoutChanged` push to all clients.

---

### Task 2: Strip client-side layout authority

**Files:**
- Modify: `crates/workspace/src/pane_group.rs`
- Modify: `crates/workspace/src/pane.rs`

- [ ] **Step 1: Remove local authoritative layout state**

Delete the split tree management code from client-side `pane_group.rs`. The client no longer holds the authoritative layout.

- [ ] **Step 2: Replace with layout projection**

Client workspace receives `LayoutTree` from server (via attach snapshot or layout-changed notification). Projects it into GPUI element positions:

```rust
fn render_layout(layout: &LayoutTree, panes: &HashMap<PaneId, Entity<dyn Item>>) -> impl IntoElement {
    match &layout.root {
        LayoutNode::Pane(leaf) => render_pane(&panes[&leaf.pane_id]),
        LayoutNode::Split(split) => render_split(split, panes),
    }
}
```

- [ ] **Step 3: Forward user interactions as RPCs**

Drag divider → `domain.adjust_layout(AdjustRequest)` → server recomputes → push → client re-renders.

---

### Task 3: Tabbar layout (top vs stacked, runtime switchable)

**Files:**
- Modify: `crates/workspace/src/pane.rs` (tabbar rendering)

- [ ] **Step 1: Implement top tabbar mode**

Standard horizontal tab strip above panes.

- [ ] **Step 2: Implement stacked tabbar mode (left dock)**

Vertical stack of tabs in left dock. Each pane = one tab entry.

- [ ] **Step 3: Implement runtime switching**

Setting `tabbar_style = "top" | "stacked"`. Change at runtime triggers workspace re-render. Not hardcoded — any workspace can switch.

---

### Task 4: Right pane (file tree / diff view as normal pane)

- [ ] **Step 1: Verify right-side panes use standard pane interaction model**

File tree and diff view open as normal panes (top tabbar). They support: split, drag-resize, multi-tab, same as terminal panes. No special-casing.

- [ ] **Step 2: Auto-split-right behavior**

When user clicks a file in file tree with only a center terminal pane open → server creates a right-side split automatically.

---

### Task 5: Tests

- [ ] **Step 1: Layout engine unit tests (server-side)**

split, close, resize, focus operations on the tree. Verify tree invariants (no orphan panes, ratios sum to 1.0).

- [ ] **Step 2: Layout projection test (client-side)**

Feed a mock LayoutTree → verify correct GPUI element positions computed.

- [ ] **Step 3: Commit**

```bash
git add crates/mux_server/src/layout.rs crates/workspace/src/ Cargo.toml
git commit -m "Migrate layout authority to server, client becomes layout renderer"
```
