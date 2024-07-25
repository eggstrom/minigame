use std::{sync::Arc, thread};

use sdl2::{render::WindowCanvas, Sdl};

use crate::{
    audio::AudioModule, event::EventModule, window::WindowModule, GameHandle, SharedState,
    WorldModule,
};

pub struct Game {
    title: Option<String>,
    size: Option<(u32, u32)>,
    accelerated: bool,
    vsync: bool,
}

macro_rules! uninitialized_fields {
    ($obj:ident, $($field:ident),+) => {
        {
            let mut uninitialized = vec![];
            $(
                if let None = $obj.$field {
                    uninitialized.push(stringify!($field));
                }
            )+
            uninitialized
        }
    };
}

impl Game {
    pub fn new() -> Self {
        Game {
            title: None,
            size: None,
            accelerated: false,
            vsync: false,
        }
    }

    pub fn start(&mut self, init: fn(&mut GameHandle)) -> Result<(), String> {
        self.validate()?;
        let sdl = sdl2::init()?;
        let canvas = self.make_canvas(&sdl)?;
        let texture_creator = canvas.texture_creator();

        let mut event_module = EventModule::new(sdl.event_pump()?);
        let mut window_module = WindowModule::new(canvas, &texture_creator)?;
        let mut audio_module = AudioModule::new();
        let state = Arc::new(SharedState::new());

        let state2 = Arc::clone(&state);

        let world_thread = thread::spawn(move || -> Result<(), String> {
            let mut world = WorldModule::new(20);
            world.start(Arc::clone(&state2), init);

            while state2.running() {
                world.update(Arc::clone(&state2)).map_err(|e| {
                    state2.stop();
                    e
                })?;
            }
            Ok(())
        });

        while state.running() {
            event_module.update(&state)?;
            audio_module.update(&state)?;
            window_module.update(&state)?;
        }

        world_thread
            .join()
            .map_err(|_| "couldn't join world thread")??;
        Ok(())
    }

    fn validate(&self) -> Result<(), String> {
        let uninitialized = uninitialized_fields!(self, size, title);

        if uninitialized.is_empty() {
            Ok(())
        } else {
            Err(format!("Game requires: {}", uninitialized.join(", ")))
        }
    }

    fn make_canvas(&self, sdl: &Sdl) -> Result<WindowCanvas, String> {
        let c = sdl
            .video()?
            .window(
                self.title.as_ref().unwrap(),
                self.size.unwrap().0,
                self.size.unwrap().1,
            )
            .build()
            .map_err(|e| e.to_string())?
            .into_canvas();
        let c = if self.vsync { c.present_vsync() } else { c };
        let c = if self.accelerated { c.accelerated() } else { c };
        c.build().map_err(|e| e.to_string())
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn size(&mut self, width: u32, height: u32) -> &mut Self {
        self.size = Some((width, height));
        self
    }

    pub fn accelerated(&mut self) -> &mut Self {
        self.accelerated = true;
        self
    }

    pub fn vsync(&mut self) -> &mut Self {
        self.vsync = true;
        self
    }
}
