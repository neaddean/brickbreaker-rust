use ggez::{ContextBuilder, event};
use ggez::{Context, GameResult, graphics};
use ggez::nalgebra as na;
use specs::{
    Builder, Component, Dispatcher, DispatcherBuilder, join::Join, ReadStorage, RunNow, System,
    VecStorage, World, WorldExt, WriteStorage,
};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    texture: graphics::Image,
}

impl Renderable {
    pub fn from_path(ctx: &mut Context, path: &str) -> Self {
        let texture = graphics::Image::new(ctx, path).unwrap();
        Renderable { texture }
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Position {
    x: f32,
    y: f32,
    z: i32,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Velocity {
    x: f32,
    y: f32,
}

pub struct RenderingSystem<'a> {
    ctx: &'a mut Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        graphics::clear(self.ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        let (positions, renderables) = data;
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        for (position, renderable) in rendering_data.iter() {
            let draw_params =
                graphics::DrawParam::new().dest(na::Point2::new(position.x, position.y));
            graphics::draw(self.ctx, &renderable.texture, draw_params).unwrap();
        }

        graphics::present(self.ctx).unwrap();
    }
}

struct PhysicsSystem;

// System implementation
impl<'a> System<'a> for PhysicsSystem {
    // Data
    type SystemData = (WriteStorage<'a, Position>, WriteStorage<'a, Velocity>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions, mut velocities) = data;

        for (position, velocity) in (&mut positions, &mut velocities).join() {
            position.x += velocity.x;
            position.y += velocity.y;

            if position.x > 800.0 {
                position.x = 800.0;
                velocity.x *= -1.0;
            } else if position.x < 0.0 {
                position.x = 0.0;
                velocity.x *= -1.0;
            }
            position.y += velocity.y;
            if position.y > 600.0 {
                position.y = 600.0;
                velocity.y *= -1.0;
            } else if position.y < 0.0 {
                position.y = 0.0;
                velocity.y *= -1.0;
            }
        }
    }
}

fn create_ball(world: &mut World, ctx: &mut Context) {
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
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Renderable>();

    create_ball(&mut world, ctx);

    let ref mut game = Game { world };

    event::run(ctx, event_loop, game).unwrap();
}
