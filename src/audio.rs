use crate::{AudioRequest, SharedState};

pub struct AudioModule {}

impl AudioModule {
    pub fn new() -> Self {
        AudioModule {}
    }

    pub fn update(&mut self, state: &SharedState) -> Result<(), String> {
        for request in state.take_audio_requests()? {
            self.handle_request(&request)?;
        }
        Ok(())
    }

    fn handle_request(&mut self, request: &AudioRequest) -> Result<(), String> {
        let _ = request;
        Ok(())
    }
}
