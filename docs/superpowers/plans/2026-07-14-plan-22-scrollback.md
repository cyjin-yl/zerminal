# Plan 22: Scrollback — Per-Client + Sync + Fetch Protocol

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans.

**Goal:** Implement scrollback viewing: per-client scroll (default) + session-wide sync scroll (collaborative, switchable via keybinding). Auto-jump to bottom on new output. On-demand fetch with client-side cache. Scrollback search (regex).

**Dependencies:** `mux`, `mux_server`, `mux_protocol`.

---

### Task 1: Server scrollback storage + fetch

**Files:**
- Modify: `crates/mux_server/src/pane.rs`

- [ ] **Step 1: Expose alacritty scrollback history via fetch_scrollback RPC**

alacritty terminal has a `history` field (ring buffer of lines above viewport). Implement `FetchScrollbackRequest` handler: read N lines from `from_line` in direction.

- [ ] **Step 2: Implement scrollback_version**

`scrollback_version = (u64 counter, u64 unix_timestamp_seconds)`. Increment when ring buffer wraps. Returned with every fetch response for cache invalidation.

- [ ] **Step 3: Implement scrollback search**

`SearchScrollbackRequest { pane_id, regex, from_line, direction }` → server searches scrollback lines → returns matching line numbers + context.

---

### Task 2: Client scrollback state

**Files:**
- Modify: `crates/terminal_view/src/terminal_view.rs`

- [ ] **Step 1: Implement per-client scroll offset**

```rust
pub struct TerminalViewState {
    scroll_offset: Option<usize>,  // None = pinned to bottom
    scrollback_cache: LruCache<(usize), Vec<Cell>>,  // from_line → cached rows
    scrollback_version: Option<(u64, u64)>,
    scroll_mode: ScrollMode,
}

pub enum ScrollMode {
    PerClient,   // default: independent scroll per client
    SessionSync, // collaborative: scroll synchronized across clients
}
```

- [ ] **Step 2: Implement scroll wheel handling**

Scroll up → enter scrollback mode → fetch from server if cache miss → render. Scroll down → if reaching bottom → exit scrollback mode → pinned to bottom.

- [ ] **Step 3: Implement auto-jump to bottom on PaneDirty**

When `PaneDirty` notification received AND user is in scrollback AND not locked → jump to bottom.

- [ ] **Step 4: Implement scroll lock toggle**

Keybinding (e.g. Ctrl-Shift-S) toggles "stay in scrollback even on new output" mode.

---

### Task 3: Session-wide sync scroll

**Files:**
- Modify: `crates/mux_server/src/session.rs`

- [ ] **Step 1: Track session-level scroll offset when in sync mode**

When a client enables session-sync scroll → notify server → server broadcasts scroll position to all attached clients → all clients jump to same offset.

- [ ] **Step 2: Keybinding to toggle between per-client and session-sync**

---

### Task 4: Cache invalidation

- [ ] **Step 1: Check scrollback_version on every fetch**

If version changed (ring wrapped) → clear entire cache → re-fetch.

---

### Task 5: Tests + Commit

- [ ] **Step 1: Scrollback fetch + cache test**

- [ ] **Step 2: Auto-jump to bottom test**

- [ ] **Step 3: Session-sync scroll test (two mock clients)**

- [ ] **Step 4: Scrollback search test**

- [ ] **Step 5: Commit**

```bash
git add crates/mux_server/src/pane.rs crates/terminal_view/src/terminal_view.rs
git commit -m "Add scrollback: per-client + sync scroll, fetch protocol, cache, search"
```
