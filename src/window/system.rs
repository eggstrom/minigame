use std::{collections::HashMap, rc::Rc, time::Instant};

use sdl2::{
    gfx::primitives::DrawRenderer,
    image::LoadTexture,
    pixels::Color,
    render::{Texture, TextureCreator, WindowCanvas},
    video::{FullscreenType, WindowContext},
};

use crate::SharedState;

use super::{DrawData, WindowRequest};

pub struct WindowSystem<'a> {
    canvas: WindowCanvas,
    texture_creator: &'a TextureCreator<WindowContext>,
    texture_cache: HashMap<&'a str, Rc<Texture<'a>>>,
    background_color: Color,
}

impl<'a> WindowSystem<'a> {
    pub fn new(
        canvas: WindowCanvas,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Self, String> {
        Ok(WindowSystem {
            canvas,
            texture_creator,
            texture_cache: HashMap::new(),
            background_color: Color::BLACK,
        })
    }

    pub fn update(&mut self, state: &SharedState) -> Result<(), String> {
        let instant = Instant::now();

        if let Some(draw_data) = state.lock_draw_data()? {
            self.canvas.set_draw_color(self.background_color);
            self.canvas.clear();

            for draw_data in draw_data.iter() {
                self.draw(draw_data)?;
            }
        }

        log::info!("Window update took {}us", instant.elapsed().as_micros());

        self.canvas.present();
        Ok(())
    }

    pub fn handle_request(&mut self, request: &WindowRequest) -> Result<(), String> {
        match request {
            WindowRequest::EnableFullscreen => self
                .canvas
                .window_mut()
                .set_fullscreen(FullscreenType::True)?,
            WindowRequest::EnableDesktopFullscreen => self
                .canvas
                .window_mut()
                .set_fullscreen(FullscreenType::Desktop)?,
            WindowRequest::Resize(w, h) => self
                .canvas
                .window_mut()
                .set_size(*w, *h)
                .map_err(|e| e.to_string())?,
            WindowRequest::DisableFullscreen => self
                .canvas
                .window_mut()
                .set_fullscreen(FullscreenType::Off)?,
            WindowRequest::SetBackgroundColor(color) => self.background_color = *color,
            // WindowRequest::LoadTexture { id, path } => self.load_texture(id, path)?,
            // WindowRequest::LoadTextureBytes { id, bytes } => self.load_texture_bytes(id, bytes)?,
        };
        Ok(())
    }

    fn draw(&mut self, draw_data: &DrawData) -> Result<(), String> {
        match draw_data {
            DrawData::Rectangle { rect, color } => {
                self.canvas.set_draw_color(*color);
                self.canvas.draw_rect(*rect)?;
            }
            DrawData::FilledRectangle { rect, color } => {
                self.canvas.set_draw_color(*color);
                self.canvas.fill_rect(*rect)?;
            }
            DrawData::Circle { x, y, rad, color } => {
                self.canvas.circle(*x, *y, *rad, *color)?;
            }
            DrawData::FilledCircle { x, y, rad, color } => {
                self.canvas.filled_circle(*x, *y, *rad, *color)?;
            }
            DrawData::Texture { id, src, dst } => {
                let texture = self.get_texture(&id)?;
                self.canvas.copy(&texture, *src, *dst)?;
            }
            DrawData::TextureEx {
                id,
                src,
                dst,
                center,
                angle,
                flip_h,
                flip_v,
            } => {
                let texture = self.get_texture(&id)?;
                self.canvas
                    .copy_ex(&texture, *src, *dst, *angle, *center, *flip_h, *flip_v)?;
            }
        }
        Ok(())
    }

    pub fn load_texture(&mut self, id: &'a str, path: &str) -> Result<(), String> {
        if self.texture_cache.contains_key(id) {
            return Err(format!("texture already exists: {id}"));
        }

        let texture = self.texture_creator.load_texture(path)?;
        self.texture_cache.insert(id, Rc::new(texture));
        Ok(())
    }

    pub fn load_texture_bytes(&mut self, id: &'a str, bytes: &[u8]) -> Result<(), String> {
        if self.texture_cache.contains_key(id) {
            return Err(format!("texture already exists: {id}"));
        }

        let texture = self.texture_creator.load_texture_bytes(bytes)?;
        self.texture_cache.insert(id, Rc::new(texture));
        Ok(())
    }

    pub fn unload_texture(&mut self, id: &str) -> Result<(), String> {
        match self.texture_cache.remove(id) {
            Some(_) => Ok(()),
            None => Err(format!("texture not found: {id}")),
        }
    }

    fn get_texture(&self, id: &str) -> Result<Rc<Texture<'a>>, String> {
        match self.texture_cache.get(id) {
            Some(texture) => Ok(Rc::clone(texture)),
            None => Err(format!("texture not found: {id}")),
        }
    }
}
