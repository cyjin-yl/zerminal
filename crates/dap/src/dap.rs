pub mod adapters;
pub mod client;
pub mod debugger_settings;
pub mod inline_value;
pub mod proto_conversions;
mod registry;
pub mod transport;

use std::net::IpAddr;

pub use dap_types::*;
use debugger_settings::DebuggerSettings;
use gpui::App;
pub use registry::{DapLocator, DapRegistry};
use serde::Serialize;
use settings::Settings;
pub use task::DebugRequest;

pub type ScopeId = u64;
pub type VariableReference = u64;
pub type StackFrameId = u64;

#[cfg(any(test, feature = "test-support"))]
pub use adapters::FakeAdapter;
use task::{DebugScenario, TcpArgumentsTemplate};

pub async fn configure_tcp_connection(
    tcp_connection: TcpArgumentsTemplate,
) -> anyhow::Result<(IpAddr, u16, Option<u64>)> {
    let host = tcp_connection.host();
    let timeout = tcp_connection.timeout;

    let port = if let Some(port) = tcp_connection.port {
        port
    } else {
        transport::TcpTransport::port(&tcp_connection).await?
    };

    Ok((host, port, timeout))
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TelemetrySpawnLocation {
    Gutter,
    ScenarioList,
    Custom,
}

pub fn send_telemetry(scenario: &DebugScenario, location: TelemetrySpawnLocation, cx: &App) {
    let Some(adapter) = cx.global::<DapRegistry>().adapter(&scenario.adapter) else {
        return;
    };
    let dock = DebuggerSettings::get_global(cx).dock;
    let config = scenario.config.clone();
    let with_build_task = scenario.build.is_some();
    let adapter_name = scenario.adapter.clone();
    cx.spawn(async move |_| {
        let kind = adapter
            .request_kind(&config)
            .await
            .ok()
            .map(serde_json::to_value)
            .and_then(Result::ok);

        telemetry::event!(
            "Debugger Session Started",
            spawn_location = location,
            with_build_task = with_build_task,
            kind = kind,
            adapter = adapter_name,
            dock_position = dock,
        );
    })
    .detach();
}
// 来源: spec §8.2 Pass 1 — dap crate 在 Plan 4 中被删除，临时恢复并标记为迁移洞

use zerminal_macros::zerminal_todo;

#[zerminal_todo("removed-crate", "dap crate 已被删除，等待恢复")]
pub struct __ZerminalTodoMarker;
