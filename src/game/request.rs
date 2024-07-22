use crate::{audio::AudioRequest, window::WindowRequest, world::WorldRequest};

#[derive(Debug)]
pub enum GameRequest {
    AudioRequest(AudioRequest),
    WindowRequest(WindowRequest),
    WorldRequest(WorldRequest),
    Stop,
}

impl From<AudioRequest> for GameRequest {
    fn from(value: AudioRequest) -> Self {
        GameRequest::AudioRequest(value)
    }
}

impl From<WindowRequest> for GameRequest {
    fn from(value: WindowRequest) -> Self {
        GameRequest::WindowRequest(value)
    }
}

impl From<WorldRequest> for GameRequest {
    fn from(value: WorldRequest) -> Self {
        GameRequest::WorldRequest(value)
    }
}
