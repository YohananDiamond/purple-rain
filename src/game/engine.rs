use crate::math::Point2;
use sdl2::render::{Canvas, RenderTarget};

pub struct GameInfo {
    pub win_size: Point2<u32>,
}

pub struct UpdateResponse {
    pub should_delete: bool,
}

impl Default for UpdateResponse {
    fn default() -> Self {
        Self {
            should_delete: false,
        }
    }
}

pub trait Entity<T: RenderTarget> {
    // TODO: maybe make this generic over a type of GameInfo
    fn update(&mut self, info: &GameInfo) -> UpdateResponse;
    fn render(&self, canvas: &mut Canvas<T>, info: &GameInfo);
}
