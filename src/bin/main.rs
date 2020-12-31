use ggez::ContextBuilder;
use specs::{DispatcherBuilder, World, WorldExt};

use bricktest::{
    components, entities,
    systems::PhysicsSystem,
};

fn main() {
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("{:?}", path);
        path
    } else {
        std::path::PathBuf::from("./resources")
    };

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("bricks", "dean")
        .add_resource_path(resource_dir)
        .build()
        .unwrap();

    let ref mut world = World::new();
    world.register::<components::Position>();
    world.register::<components::Velocity>();
    world.register::<components::Renderable>();

    entities::create_ball(world, ctx);

    // let ref mut game = Game { world };

    let ref mut dispatcher = DispatcherBuilder::new()
        .with(PhysicsSystem, "physics", &[])
        .build();

    dispatcher.setup(world);

    bricktest::gameloop::run(ctx, event_loop, dispatcher, world);

    // event::run(ctx, event_loop, game).unwrap();
}
