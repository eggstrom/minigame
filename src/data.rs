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
    audio_requests: Mutex<Vec<AudioRequest>>,
    window_requests: Mutex<Vec<WindowRequest>>,
    draw_data: Mutex<Vec<DrawData>>,
    new_draw_data: AtomicBool,
}

impl SharedState {
    pub fn new() -> Self {
        SharedState {
            running: true.into(),
            events: vec![].into(),
            audio_requests: vec![].into(),
            window_requests: vec![].into(),
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

    pub fn send_audio_request(&self, request: AudioRequest) -> Result<(), String> {
        self.audio_requests
            .lock()
            .map_err(|e| e.to_string())?
            .push(request);
        Ok(())
    }

    pub fn send_window_request(&self, request: WindowRequest) -> Result<(), String> {
        self.window_requests
            .lock()
            .map_err(|e| e.to_string())?
            .push(request);
        Ok(())
    }

    pub fn take_audio_requests(&self) -> Result<Vec<AudioRequest>, String> {
        Ok(mem::take(
            &mut *self.audio_requests.lock().map_err(|e| e.to_string())?,
        ))
    }

    pub fn take_window_requests(&self) -> Result<Vec<WindowRequest>, String> {
        Ok(mem::take(
            &mut *self.window_requests.lock().map_err(|e| e.to_string())?,
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

pub trait GameRequest {
    fn send(self, state: &SharedState) -> Result<(), String>;
}

impl GameRequest for AudioRequest {
    fn send(self, state: &SharedState) -> Result<(), String> {
        state.send_audio_request(self)
    }
}

impl GameRequest for WindowRequest {
    fn send(self, state: &SharedState) -> Result<(), String> {
        state.send_window_request(self)
    }
}

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
