use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<crate::events::Event>,
}

pub type EntityQueue = Vec<crate::entities::EntityType>;

pub struct GameState {
    pub show_fps: bool,
    pub show_debug: bool,
    pub continuing: bool,
    pub sw_frame_limiter: bool,
    pub screen_size: (f32, f32),
    this_duration: Duration,
    this_instant: Instant,
    last_instant: Instant,
}

impl GameState {
    pub fn new(ctx: &mut ggez::Context) -> Self {
        GameState {
            this_instant: Instant::now(),
            last_instant: Instant::now(),
            show_fps: true,
            show_debug: true,
            continuing: true,
            sw_frame_limiter: false,
            screen_size: ggez::graphics::drawable_size(&ctx),
            this_duration: Default::default(),
        }
    }
}

impl GameState {
    pub fn tick(&mut self) {
        self.this_instant = Instant::now();
        self.this_duration = self.this_instant.duration_since(self.last_instant);
        self.last_instant = self.this_instant;
    }

    pub fn this_duration(&self) -> Duration {
        self.this_duration
    }
}

#[derive(Default)]
pub struct AssetCache {
    pub cache: HashMap<String, ggez::graphics::Image>,
}

impl AssetCache {
    pub fn load_assets(&mut self, ctx: &mut ggez::Context) {
        for path in ggez::filesystem::read_dir(ctx, "/")
            .unwrap()
            .filter(|p| p.to_str().unwrap().ends_with(".png"))
        {
            println!("Loading asset: {}", path.to_str().unwrap());
            self.cache.insert(
                String::from(path.to_str().unwrap()),
                ggez::graphics::Image::new(ctx, path).unwrap(),
            );
        }
    }
}
