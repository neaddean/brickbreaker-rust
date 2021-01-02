use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub asset_name: String,
}


#[derive(Component)]
#[storage(VecStorage)]
pub struct Ball;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Bar;


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
