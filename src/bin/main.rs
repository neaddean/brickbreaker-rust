use std::collections::HashMap;

use ggez::ContextBuilder;
use specs::{DispatcherBuilder, World, WorldExt};

use bricktest::{components, entities, systems::{EntityCreatorSystem, EventSystem, PhysicsSystem}};
use bricktest::resources::{AssetCache, EntityQueue, GameState};

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
        .with(EntityCreatorSystem, "entites", &["events"])
        .with(PhysicsSystem, "physics", &["entites"])
        .build();

    dispatcher.setup(world);

    {
        let mut entity_queue = world.write_resource::<EntityQueue>();
        entity_queue.push(entities::EntityType::Ball { x: 1.0, y: 2.0 });
        entity_queue.push(entities::EntityType::Ball { x: 0.5, y: 0.75 });
        entity_queue.push(entities::EntityType::Ball { x: -0.25, y: 1.75 });
        entity_queue.push(entities::EntityType::Ball { x: 2.25, y: 1.33 });
        entity_queue.push(entities::EntityType::Bar);
    }

    world.insert(AssetCache { cache: HashMap::new() });

    {
        let mut asset_cache = world.write_resource::<AssetCache>();
        asset_cache.load_assets(ctx);
    }

    {
        let mut game_state = world.write_resource::<GameState>();
        game_state.show_fps = true;
        game_state.continuing = true;
    }

    bricktest::gameloop::run(ctx, event_loop, dispatcher, world);
}
