use ggez::{ContextBuilder, event};
use ggez::{Context, GameResult, graphics};
use ggez::nalgebra as na;
use specs::{
    Builder, Component, Dispatcher, DispatcherBuilder, join::Join, ReadStorage, RunNow, System,
    VecStorage, World, WorldExt, WriteStorage,
};

use crate::components::*;


pub fn create_ball(world: &mut World, ctx: &mut Context) {
    world
        .create_entity()
        .with(Position {
            x: 0.0,
            y: 0.0,
            z: 0,
        })
        .with(Velocity{x : 2.0, y: 2.0})
        .with(Renderable::from_path(ctx, "/ball.png"))
        .build();
}