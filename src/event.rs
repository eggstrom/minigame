use sdl2::{
    event::Event,
    keyboard::{Keycode, Mod, Scancode},
    EventPump,
};

use crate::SharedState;

pub struct EventModule {
    event_pump: EventPump,
}

impl EventModule {
    pub fn new(event_pump: EventPump) -> Self {
        EventModule { event_pump }
    }

    pub fn update(&mut self, state: &SharedState) -> Result<(), String> {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => state.stop(),
                _ => {
                    if let Ok(event) = event.try_into() {
                        state.push_event(event)?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn is_pressed(&self, keycode: Keycode) -> Result<bool, String> {
        Ok(self.event_pump.keyboard_state().is_scancode_pressed(
            Scancode::from_keycode(keycode)
                .ok_or(format!("couldn't convert keycode {keycode} to scancode"))?,
        ))
    }
}

pub enum EventData {
    KeyDown {
        keycode: Keycode,
        scancode: Scancode,
        keymod: Mod,
    },
}

impl TryFrom<Event> for EventData {
    type Error = ();

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        match value {
            Event::KeyDown {
                keycode: Some(keycode),
                scancode: Some(scancode),
                keymod,
                ..
            } => Ok(EventData::KeyDown {
                keycode,
                scancode,
                keymod,
            }),
            _ => Err(()),
        }
    }
}
