use std::cell::RefCell;
use std::rc::Rc;

use ggez::ContextBuilder;
use specs::{DispatcherBuilder, World, WorldExt};

use bricktest::{entities, systems::{EntityCreatorSystem, EventSystem, PhysicsSystem}};
use bricktest::resources::{AssetCache, EntityQueue, GameState};
use bricktest::systems::{InputSystem, RenderingSystem};

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

    let ctx = Rc::new(RefCell::new(ctx));

    let ref mut world = World::new();
    world.insert(GameState::new(*ctx.borrow_mut()));

    let ref mut dispatcher = DispatcherBuilder::new()
        .with(EventSystem, "events", &[])
        .with(EntityCreatorSystem, "entites", &["events"])
        .with(PhysicsSystem::default(), "physics", &["entites"])
        .with_thread_local(InputSystem { ctx: Rc::clone(&ctx), event_loop })
        .with_thread_local(RenderingSystem::new(Rc::clone(&ctx)))
        .build();

    dispatcher.setup(world);

    {
        let mut entity_queue = world.write_resource::<EntityQueue>();
        entity_queue.push(entities::EntityType::Ball { x: 60.0, y: 100.0 });
        entity_queue.push(entities::EntityType::Ball { x: 25.0, y: 75.0 });
        entity_queue.push(entities::EntityType::Ball { x: -15.0, y: 90.0 });
        entity_queue.push(entities::EntityType::Ball { x: -130.0, y: 20.0 });
        entity_queue.push(entities::EntityType::Bar);

        entity_queue.push(entities::EntityType::Brick { x: 50.0, y: 50.0, health: 2 });
        entity_queue.push(entities::EntityType::Brick { x: 500.0, y: 500.0, health: 2 });
    }
    {
        let mut asset_cache = world.write_resource::<AssetCache>();
        asset_cache.load_assets(*ctx.borrow_mut());
    }
    bricktest::gameloop::run(dispatcher, world);
}
