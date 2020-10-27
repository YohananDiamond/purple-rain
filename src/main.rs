use std::thread;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::RenderTarget;
use sdl2::video::Window;

use rand::Rng;

pub mod game;
use game::engine::{Entity, GameInfo};
use game::entities;

pub mod math;
use math::Point2;

struct GameState<'a> {
    pub entities: Vec<EntityWrapper<'a, Window>>,
}

impl<'a> GameState<'a> {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn add_entity<T: Entity<Window> + 'a>(&mut self, entity: T) {
        self.entities.push(EntityWrapper::new(entity));
    }
}

struct EntityWrapper<'a, T> {
    inner: Box<dyn Entity<T> + 'a>,
    delete_next: bool,
}

impl<'a, T: RenderTarget> EntityWrapper<'a, T> {
    pub fn new<E: Entity<T> + 'a>(entity: E) -> Self {
        Self {
            inner: Box::new(entity),
            delete_next: false,
        }
    }
}

struct Game {}

impl Game {
    pub fn start(&mut self) -> Result<(), String> {
        const FRAMERATE: u32 = 60;
        const WIN_SIZE: (u32, u32) = (800, 600);

        let mut rng = rand::thread_rng();
        let frame_min_time = Duration::from_secs(1) / FRAMERATE;
        let context = sdl2::init()?;
        let video = context.video()?;

        let window = match video
            .window("DEMO", WIN_SIZE.0, WIN_SIZE.1)
            .position_centered()
            .build()
        {
            Ok(window) => window,
            Err(e) => return Err(format!("{}", e)),
        };

        let mut canvas = match window.into_canvas().build() {
            Ok(canvas) => canvas,
            Err(e) => return Err(format!("{}", e)),
        };
        let mut event_pump = context.event_pump()?;

        let mut running = true;
        let mut state = GameState::new();
        let info = GameInfo {
            win_size: Point2::from(WIN_SIZE),
        };

        const RAINDROP_TIMER_CYCLE: u8 = 1;
        let mut raindrop_timer = RAINDROP_TIMER_CYCLE;

        while running {
            let now = Instant::now();

            // clear screen
            canvas.set_draw_color(Color::RGB(10, 20, 5));
            canvas.clear();

            for ew in state.entities.iter_mut() {
                let response = ew.inner.update(&info);

                if response.should_delete {
                    ew.delete_next = true;
                } else {
                    ew.inner.render(&mut canvas, &info);
                    ew.delete_next = false;
                }
            }

            const RAINDROP_MIN_INITIAL_FALL_SPEED: f64 = 5.0;
            const RAINDROP_MAX_INITIAL_FALL_SPEED: f64 = 10.0;

            if raindrop_timer <= 0 {
                let mut do_thing = || {
                    state.add_entity(entities::Raindrop {
                        speed: Point2 {
                            x: 0.0,
                            y: RAINDROP_MIN_INITIAL_FALL_SPEED + rng.gen::<f64>() * (RAINDROP_MAX_INITIAL_FALL_SPEED - RAINDROP_MIN_INITIAL_FALL_SPEED).max(0.0),
                        },
                        pos: Point2 {
                            x: rng.gen::<f64>() * (info.win_size.x - entities::RAINDROP_RECT_SIZE.0) as f64,
                            y: -(entities::RAINDROP_RECT_SIZE.1 as f64),
                        },
                    });
                };

                do_thing();
                do_thing();

                raindrop_timer = RAINDROP_TIMER_CYCLE;
            } else {
                raindrop_timer -= 1;
            }

            state.entities.retain(|ew| !ew.delete_next);

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => running = false,
                    _ => (),
                }
            }

            canvas.present();

            let elapsed = now.elapsed();
            if elapsed < frame_min_time {
                thread::sleep(frame_min_time - elapsed);
            }
        }

        Ok(())
    }
}

pub fn main() {
    let mut game = Game {};
    game.start().unwrap();
}
