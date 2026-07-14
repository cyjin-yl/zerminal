mod channel_buffer;
mod channel_store;

use client::{Client, UserStore};
use gpui::{App, Entity};
use std::sync::Arc;

pub use channel_buffer::{ACKNOWLEDGE_DEBOUNCE_INTERVAL, ChannelBuffer, ChannelBufferEvent};
pub use channel_store::{Channel, ChannelEvent, ChannelMembership, ChannelStore};

#[cfg(test)]
mod channel_store_tests;

pub fn init(client: &Arc<Client>, user_store: Entity<UserStore>, cx: &mut App) {
    channel_store::init(client, user_store, cx);
    channel_buffer::init(&client.clone().into());
}
// 来源: spec §8.2 Pass 1 — channel crate 在 Plan 4 中被删除，临时恢复并标记为迁移洞

use zerminal_macros::zerminal_todo;

#[zerminal_todo("removed-crate", "channel crate 已被删除，等待恢复")]
pub struct __ZerminalTodoMarker;
