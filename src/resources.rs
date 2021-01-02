use std::collections::HashMap;

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<crate::events::Event>,
}

pub type EntityQueue = Vec<crate::entities::EntityType>;

#[derive(Default)]
pub struct GameState {
    pub pending_updates: u8,
    pub show_fps: bool,
    pub continuing: bool,
    pub screen_size: (f32, f32),
}

#[derive(Default)]
pub struct AssetCache {
    pub cache: HashMap<String, ggez::graphics::Image>,
}

impl AssetCache {
    pub fn load_assets(&mut self, ctx: &mut ggez::Context) {
        for path in ggez::filesystem::read_dir(ctx, "/")
            .unwrap()
            .filter(|p| p.to_str().unwrap().ends_with(".png")) {
            println!("Loading asset: {}", path.to_str().unwrap());
            self.cache.insert(String::from(path.to_str().unwrap()),
                              ggez::graphics::Image::new(ctx, path).unwrap());
        }
    }
}
