use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod, Scancode};

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
