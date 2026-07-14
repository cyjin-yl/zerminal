# Plan 20: Clipboard — Server Relay Hub

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans.

**Goal:** Implement server-canonical clipboard: mux_server maintains a clipboard space supporting text/image/file-path with origin_host metadata. OSC 52 and bracketed paste integrated. Multi-client: no auto-mirror, only actual copy triggers sync.

**Dependencies:** `mux_server`, `mux_protocol`, `gpui`.

---

### Task 1: Server clipboard space

**Files:**
- Modify: `crates/mux_server/src/clipboard.rs`

- [ ] **Step 1: Implement ClipboardEntry storage**

```rust
pub struct ServerClipboard {
    current: parking_lot::RwLock<Option<ClipboardEntry>>,
}

pub struct ClipboardEntry {
    pub content_type: ClipboardContentType,
    pub data: Vec<u8>,
    pub origin_host: String,
}

pub enum ClipboardContentType {
    Text,
    ImagePng,
    FilePath,
}
```

- [ ] **Step 2: Implement SetClipboard / GetClipboard RPC handlers**

Server receives `SetClipboardRequest` → updates current → pushes `ClipboardChanged` notification to all clients. `GetClipboardRequest` returns current entry.

---

### Task 2: OSC 52 integration

**Files:**
- Modify: `crates/mux_server/src/pane.rs` (terminal emulator event handling)

- [ ] **Step 1: Parse OSC 52 in server emulator**

alacritty terminal emits OSC 52 sequences. Server intercepts → base64-decode content → write to ServerClipboard → push notification to clients.

- [ ] **Step 2: Client writes to system clipboard on receipt**

Client receives `ClipboardChanged` notification → reads from server clipboard → writes to local system clipboard (via GPUI clipboard API).

---

### Task 3: Bracketed paste

**Files:**
- Modify: `crates/mux_server/src/pane.rs`

- [ ] **Step 1: Server tracks bracketed paste mode**

When PTY application enables bracketed paste (`ESC [ ? 2004 h`), server's emulator tracks this state.

- [ ] **Step 2: Paste RPC wraps content if mode active**

`PasteRequest` received → server checks bracketed paste mode → if active, wraps content with `ESC [ 200 ~ ... ESC [ 201 ~` → writes to PTY.

---

### Task 4: Path forwarding

**Files:**
- Modify: `crates/mux/src/clipboard.rs`

- [ ] **Step 1: FilePath entries carry origin_host**

When clipboard entry has `content_type = FilePath`, it includes `origin_host` (which machine the path is from).

- [ ] **Step 2: Client resolves path based on origin**

If `origin_host == local` → open directly. If `origin_host == remote` → fetch via mux_protocol `ReadFileRequest`. If image path → fetch and render.

---

### Task 5: Multi-client behavior (no auto-mirror)

- [ ] **Step 1: Only actual copy triggers sync**

Client system clipboard change events are NOT monitored. Only: OSC 52 from PTY, or explicit user copy action (Ctrl-Shift-C / right-click copy).

- [ ] **Step 2: Prevent clipboard pollution**

System clipboard is only written when: (a) user explicitly copies in terminal, or (b) OSC 52 from PTY. Not on every server clipboard change.

---

### Task 6: Tests + Commit

- [ ] **Step 1: OSC 52 round-trip test**

- [ ] **Step 2: Bracketed paste wrapping test**

- [ ] **Step 3: Multi-client no-pollution test**

- [ ] **Step 4: Commit**

```bash
git add crates/mux_server/src/clipboard.rs crates/mux/src/clipboard.rs
git commit -m "Add server clipboard relay hub: OSC 52, bracketed paste, path forwarding, no auto-mirror"
```
