use ggez::{graphics, Context};
use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub texture: graphics::Image,
}

impl Renderable {
    pub fn from_path(ctx: &mut Context, path: &str) -> Self {
        let texture = graphics::Image::new(ctx, path).unwrap();
        Renderable { texture }
    }
}

#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: i32,
}

#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
