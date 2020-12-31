use ggez::ContextBuilder;
use specs::{DispatcherBuilder, World, WorldExt};

use bricktest::{components, entities, systems::{EventSystem, PhysicsSystem}};

fn main() {
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        std::path::PathBuf::from("./resources")
    };

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("bricks", "dean")
        .add_resource_path(resource_dir)
        .build()
        .unwrap();

    let ref mut world = World::new();
    world.register::<components::Renderable>();

    let ref mut dispatcher = DispatcherBuilder::new()
        .with(EventSystem, "events", &[])
        .with(PhysicsSystem, "physics", &["events"])
        .build();

    dispatcher.setup(world);

    entities::create_ball(world, ctx, 1.0, 2.0);
    entities::create_ball(world, ctx, 0.5, 0.75);
    entities::create_ball(world, ctx, -0.25, 1.75);
    entities::create_ball(world, ctx, 2.25, 1.33);

    bricktest::gameloop::run(ctx, event_loop, dispatcher, world);
}
