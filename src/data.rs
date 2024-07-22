use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
};

use std::{
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex, MutexGuard,
    },
};

use crate::event::EventData;

pub struct SharedState {
    running: AtomicBool,
    events: Mutex<Vec<EventData>>,
    requests: Mutex<Vec<GameRequest>>,
    draw_data: Mutex<Vec<DrawData>>,
    new_draw_data: AtomicBool,
}

impl SharedState {
    pub fn new() -> Self {
        SharedState {
            running: true.into(),
            events: vec![].into(),
            requests: vec![].into(),
            draw_data: vec![].into(),
            new_draw_data: true.into(),
        }
    }

    pub fn running(&self) -> bool {
        self.running.load(Ordering::Acquire)
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Release);
    }

    pub fn push_event(&self, event: EventData) -> Result<(), String> {
        self.events.lock().map_err(|e| e.to_string())?.push(event);
        Ok(())
    }

    pub fn take_events(&self) -> Result<Vec<EventData>, String> {
        Ok(mem::take(
            &mut *self.events.lock().map_err(|e| e.to_string())?,
        ))
    }

    pub fn push_requests(&self, requests: &mut Vec<GameRequest>) -> Result<(), String> {
        self.requests
            .lock()
            .map_err(|e| e.to_string())?
            .append(requests);
        Ok(())
    }

    pub fn take_requests(&self) -> Result<Vec<GameRequest>, String> {
        Ok(mem::take(
            &mut *self.requests.lock().map_err(|e| e.to_string())?,
        ))
    }

    pub fn set_draw_data(&self, draw_data: Vec<DrawData>) -> Result<(), String> {
        let _ = mem::replace(
            &mut *self.draw_data.lock().map_err(|e| e.to_string())?,
            draw_data,
        );
        self.new_draw_data.store(true, Ordering::Release);
        Ok(())
    }

    pub fn lock_draw_data(&self) -> Result<Option<MutexGuard<Vec<DrawData>>>, String> {
        if self.new_draw_data.load(Ordering::Acquire) {
            let data = self.draw_data.lock().map_err(|e| e.to_string());
            self.new_draw_data.store(false, Ordering::Release);
            data.map(|data| Some(data))
        } else {
            Ok(None)
        }
    }
}

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

#[derive(Debug)]
pub enum AudioRequest {}

#[derive(Debug)]
pub enum WindowRequest {
    DisableFullscreen,
    EnableDesktopFullscreen,
    EnableFullscreen,
    // LoadTexture{id: String, path: String},
    // LoadTextureBytes{id: String, bytes: Vec<u8>},
    Resize(u32, u32),
    SetBackgroundColor(Color),
}

#[derive(Debug)]
pub enum WorldRequest {}

#[derive(Debug)]
pub enum DrawData {
    Rectangle {
        rect: Rect,
        color: Color,
    },
    FilledRectangle {
        rect: Rect,
        color: Color,
    },
    Circle {
        x: i16,
        y: i16,
        rad: i16,
        color: Color,
    },
    FilledCircle {
        x: i16,
        y: i16,
        rad: i16,
        color: Color,
    },
    Texture {
        id: String,
        src: Option<Rect>,
        dst: Option<Rect>,
    },
    TextureEx {
        id: String,
        src: Option<Rect>,
        dst: Option<Rect>,
        center: Option<Point>,
        angle: f64,
        flip_h: bool,
        flip_v: bool,
    },
}
