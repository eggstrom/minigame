use super::AudioRequest;

pub struct AudioSystem {}

impl AudioSystem {
    pub fn new() -> Self {
        AudioSystem {}
    }

    pub fn handle_request(&mut self, request: &AudioRequest) -> Result<(), String> {
        let _ = request;
        Ok(())
    }
}
