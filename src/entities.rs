use ggez::Context;

use specs::{Builder, World, WorldExt};

use crate::components::*;

pub fn create_ball(world: &mut World, ctx: &mut Context, x: f32, y: f32) {
    world
        .create_entity()
        .with(Position {
            x: 0.0,
            y: 0.0,
            z: 0,
        })
        .with(Velocity { x, y })
        .with(Renderable::from_path(ctx, "/ball.png"))
        .build();
}
