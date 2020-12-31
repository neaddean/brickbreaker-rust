use ggez::{event, ContextBuilder};
use ggez::{Context, GameResult};

use specs::{
    join::Join, Builder, Component, Dispatcher, DispatcherBuilder, ReadStorage, RunNow, System,
    VecStorage, World, WorldExt, WriteStorage,
};

use bricktest::{
    components, entities,
    systems::{PhysicsSystem, RenderingSystem},
};

struct Game {
    world: specs::World,
}

impl ggez::event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        {
            let mut ps = PhysicsSystem;
            ps.run_now(&self.world);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        {
            let mut rs = RenderingSystem { ctx };
            rs.run_now(&self.world);
        }
        Ok(())
    }
}

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

    let mut world = World::new();
    world.register::<components::Position>();
    world.register::<components::Velocity>();
    world.register::<components::Renderable>();

    entities::create_ball(&mut world, ctx);

    let ref mut game = Game { world };

    event::run(ctx, event_loop, game).unwrap();
}
