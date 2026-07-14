use std::time::Duration;

use rodio::{ChannelCount, SampleRate, nz};

pub const REPLAY_DURATION: Duration = Duration::from_secs(30);
pub const SAMPLE_RATE: SampleRate = nz!(48000);
pub const CHANNEL_COUNT: ChannelCount = nz!(2);

mod audio_settings;
pub use audio_settings::AudioSettings;

mod audio_pipeline;
pub use audio_pipeline::Audio;
pub use audio_pipeline::{AudioDeviceInfo, AvailableAudioDevices};
pub use audio_pipeline::{ensure_devices_initialized, resolve_device};
// TODO(audio) replace with input test functionality in the audio crate
pub use audio_pipeline::RodioExt;
pub use audio_pipeline::init;
pub use audio_pipeline::{open_input_stream, open_test_output};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Sound {
    Joined,
    GuestJoined,
    Leave,
    Mute,
    Unmute,
    StartScreenshare,
    StopScreenshare,
    AgentDone,
}

impl Sound {
    fn file(&self) -> &'static str {
        match self {
            Self::Joined => "joined_call",
            Self::GuestJoined => "guest_joined_call",
            Self::Leave => "leave_call",
            Self::Mute => "mute",
            Self::Unmute => "unmute",
            Self::StartScreenshare => "start_screenshare",
            Self::StopScreenshare => "stop_screenshare",
            Self::AgentDone => "agent_done",
        }
    }
}
// 来源: spec §8.2 Pass 1 — audio crate 在 Plan 4 中被删除，临时恢复并标记为迁移洞

use zerminal_macros::zerminal_todo;

#[zerminal_todo("removed-crate", "audio crate 已被删除，等待恢复")]
pub struct __ZerminalTodoMarker;
