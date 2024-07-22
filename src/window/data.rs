use sdl2::{pixels::Color, rect::{Point, Rect}};

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
