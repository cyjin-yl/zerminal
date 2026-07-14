# Plan 11: mux — Client-Side MuxDomain

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans.

**Goal:** Create the `mux` crate — client-side `MuxDomain` struct that connects to mux_server, sends RPCs, receives notifications, and provides grid sync to the GUI.

**Architecture:** `MuxDomain` is a concrete struct (no trait). `MuxTransport` is an enum (Local/Ssh). Framed binary prost messages over `interprocess` local socket or SSH channel.

**Dependencies:** `mux_protocol`, `interprocess`, `tokio`, `prost`.

---

### Task 1: Create crate skeleton

**Files:**
- Create: `crates/mux/Cargo.toml`
- Create: `crates/mux/src/mux.rs`

- [ ] **Step 1: Create Cargo.toml**

```toml
[package]
name = "mux"
version = "0.1.0"
edition = "2024"
publish = false
license = "Apache-2.0"

[lib]
path = "src/mux.rs"

[dependencies]
mux_protocol = { workspace = true }
interprocess = "2"
tokio = { workspace = true, features = ["full"] }
prost = { workspace = true }
anyhow = { workspace = true }
parking_lot = { workspace = true }
tracing = { workspace = true }
```

- [ ] **Step 2: Add to workspace Cargo.toml**

- [ ] **Step 3: Create lib.rs with MuxDomain + MuxTransport**

```rust
use anyhow::Result;
use std::path::Path;
use tokio::sync::mpsc;
use mux_protocol::*;

pub struct MuxDomain {
    inner: parking_lot::RwLock<DomainInner>,
}

struct DomainInner {
    transport: MuxTransport,
    next_request_id: u64,
    pending_requests: std::collections::HashMap<u64, tokio::sync::oneshot::Sender<Response>>,
    notification_tx: mpsc::Sender<MuxNotification>,
}

pub enum MuxTransport {
    Local { stream: interprocess::local_socket::Stream },
    Ssh { channel: SshChannel },
}

// SshChannel is a placeholder — implemented in Plan 19 (remote connection)
pub struct SshChannel;
```

- [ ] **Step 4: Verify compilation**

Run: `cargo check -p mux`
Expected: PASS

---

### Task 2: Implement connection + I/O loop

**Files:**
- Create: `crates/mux/src/connection.rs`
- Modify: `crates/mux/src/mux.rs`

- [ ] **Step 1: Implement connect() function**

Detects local socket, connects, returns MuxDomain. Used by entry point (Plan 12).

- [ ] **Step 2: Implement background I/O task**

Reads framed Envelope messages from socket. Dispatches Responses to pending_requests via request_id match. Dispatches Notifications to notification_tx channel.

- [ ] **Step 3: Implement send_request() helper**

Serializes Request into Envelope, frames it, writes to socket. Returns oneshot::Receiver<Response> keyed by request_id.

---

### Task 3: Implement all MuxDomain methods

**Files:**
- Modify: `crates/mux/src/mux.rs`

- [ ] **Step 1: Implement session lifecycle methods**

```rust
impl MuxDomain {
    pub async fn list_sessions(&self) -> Result<Vec<SessionInfo>>;
    pub async fn create_session(&self, name: &str, cwd: &Path) -> Result<String>;
    pub async fn kill_session(&self, id: &str) -> Result<()>;
    pub async fn rename_session(&self, id: &str, name: &str) -> Result<()>;
}
```

Each sends the appropriate Request variant, awaits Response, extracts payload.

- [ ] **Step 2: Implement pane lifecycle methods**

```rust
    pub async fn spawn_pane(&self, session: &str, tab: &str, size: TerminalSize,
                            command: Option<ShellCommand>, cwd: Option<&Path>) -> Result<String>;
    pub async fn split_pane(&self, pane: &str, direction: SplitDirection) -> Result<String>;
    pub async fn close_pane(&self, pane: &str) -> Result<()>;
    pub async fn focus_pane(&self, pane: &str) -> Result<()>;
    pub async fn resize_pane(&self, pane: &str, cols: u32, rows: u32) -> Result<()>;
    pub async fn set_pane_title(&self, pane: &str, title: &str) -> Result<()>;
```

- [ ] **Step 3: Implement input methods**

```rust
    pub async fn send_input(&self, pane: &str, bytes: &[u8]) -> Result<()>;
    pub async fn paste(&self, pane: &str, text: &str) -> Result<()>;
```

- [ ] **Step 4: Implement grid sync methods**

```rust
    pub async fn fetch_grid_update(&self, pane: &str, since: u64) -> Result<FetchGridUpdateResponse>;
    pub async fn fetch_scrollback(&self, pane: &str, from: u32, direction: u32, count: u32) -> Result<FetchScrollbackResponse>;
```

- [ ] **Step 5: Implement attach/detach**

```rust
    pub async fn attach(&self, session: &str, mode: AttachMode) -> Result<AttachResponse>;
    pub async fn detach(&self) -> Result<()>;
```

- [ ] **Step 6: Implement subscribe**

```rust
    pub fn subscribe(&self) -> mpsc::Receiver<MuxNotification>;
```

Returns a new receiver. Multiple subscribers supported via broadcast or fan-out.

---

### Task 4: Unit tests

**Files:**
- Create: `crates/mux/tests/mux_domain.rs`

- [ ] **Step 1: Write mock server test**

Create a mock that listens on a local socket, accepts connection, responds to RPCs with canned responses. Verify MuxDomain sends correct requests and parses responses.

- [ ] **Step 2: Write generation counter test**

Verify fetch_grid_update correctly handles: no update (same generation), diff (within ring), full snapshot (behind ring).

- [ ] **Step 3: Run tests**

Run: `cargo test -p mux`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add crates/mux Cargo.toml
git commit -m "Add mux client crate: MuxDomain, transport, grid sync, notification subscription"
```
