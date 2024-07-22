use std::{sync::Arc, thread};

use sdl2::{render::WindowCanvas, Sdl};

use crate::{
    audio::AudioSystem, event::EventSystem, window::WindowSystem, world::World, GameRequest,
    SharedState,
};

pub struct Game {
    accelerated: bool,
    size: Option<(u32, u32)>,
    title: Option<String>,
    vsync: bool,
}

macro_rules! uninitialized_fields {
    ($obj:expr, $($field:ident),+) => {
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
            accelerated: false,
            size: None,
            title: None,
            vsync: false,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.validate()?;
        let sdl = sdl2::init()?;
        let canvas = self.make_canvas(&sdl)?;
        let texture_creator = canvas.texture_creator();

        let mut event_system = EventSystem::new(sdl.event_pump()?);
        let mut window_system = WindowSystem::new(canvas, &texture_creator)?;
        let mut audio_system = AudioSystem::new();
        let state = Arc::new(SharedState::new());

        let world_state = Arc::clone(&state);
        let world_thread = thread::spawn(move || {
            let mut world = World::new(20);

            while world_state.running() {
                if let Err(_) = world.update(&world_state) {
                    // log::error!("{e}");
                    world_state.stop();
                }
            }
        });

        while state.running() {
            for req in state.take_requests()? {
                match req {
                    GameRequest::AudioRequest(req) => audio_system.handle_request(&req)?,
                    GameRequest::WindowRequest(req) => window_system.handle_request(&req)?,
                    GameRequest::WorldRequest(_) => unreachable!(),
                    GameRequest::Stop => state.stop(),
                }
            }
            event_system.update(&state)?;
            window_system.update(&state)?;
        }

        world_thread
            .join()
            .map_err(|_| "couldn't join world thread")?;
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

    pub fn accelerated(&mut self) -> &mut Self {
        self.accelerated = true;
        self
    }

    pub fn size(&mut self, width: u32, height: u32) -> &mut Self {
        self.size = Some((width, height));
        self
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn vsync(&mut self) -> &mut Self {
        self.vsync = true;
        self
    }
}

#[macro_export]
macro_rules! type_ids {
    ($($t:ty),+) => {
        &[$(TypeId::of::<$t>()),+]
    };
}
