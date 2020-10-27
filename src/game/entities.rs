use crate::game::{Entity, GameInfo, UpdateResponse};
use crate::math::Point2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

pub const RAINDROP_GRAVITY: f64 = 0.1;
pub const RAINDROP_RECT_SIZE: (u32, u32) = (5, 20);

pub struct Raindrop {
    pub speed: Point2<f64>,
    pub pos: Point2<f64>,
}

impl<T: RenderTarget> Entity<T> for Raindrop {
    fn update(&mut self, info: &GameInfo) -> UpdateResponse {
        self.pos += self.speed;
        self.speed.y += RAINDROP_GRAVITY;

        UpdateResponse {
            should_delete: self.pos.y >= info.win_size.y as f64,
        }
    }

    fn render(&self, canvas: &mut Canvas<T>, _info: &GameInfo) {
        let half_size = Point2::from(RAINDROP_RECT_SIZE) / Point2::new(2, 2);

        canvas.set_draw_color(Color::RGB(100, 25, 130));
        canvas
            .fill_rect(Rect::new(
                self.pos.x as i32 - half_size.x as i32,
                self.pos.y as i32 - half_size.y as i32,
                RAINDROP_RECT_SIZE.0,
                RAINDROP_RECT_SIZE.1,
            ))
            .unwrap();
    }
}
