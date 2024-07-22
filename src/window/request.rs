use sdl2::pixels::Color;

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
