# Plan 32: Multiple Windows Per Session

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans.

**Goal:** Implement multiple GUI windows attached to the same session. Currently one session = one window. Allow multiple windows to share the same session state (panes, tabs, layout).

**Dependencies:** `workspace`, `mux`, `mux_server`.

**Spec:** §3.3 Mux Architecture (post-foundation)

---

### Task 1: Session window tracking

**Files:**
- Modify: `crates/mux_server/src/session.rs`

- [ ] **Step 1: Track connected windows per session**

Each session maintains a list of connected window IDs (not just client count).

- [ ] **Step 2: Broadcast layout changes to all windows**

When layout changes, notify all connected windows.

---

### Task 2: Client window management

**Files:**
- Modify: `crates/mux/src/mux.rs`

- [ ] **Step 1: Support multiple MuxDomain per session**

Allow creating multiple GUI windows that share the same session.

- [ ] **Step 2: Window lifecycle management**

Track which windows are attached to which sessions.

---

### Task 3: CLI support

- [ ] **Step 1: `z3rm new-window -t session`**

Open a new GUI window attached to an existing session.

---

### Task 4: Tests + Commit

- [ ] `cargo check` passes
- [ ] Commit + push
