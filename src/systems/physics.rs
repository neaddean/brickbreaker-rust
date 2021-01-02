use specs::{join::Join, Read, ReadStorage, System, WriteStorage};

use crate::components::*;
use crate::resources;

pub struct PhysicsSystem;

// System implementation
impl<'a> System<'a> for PhysicsSystem {
    // Data
    type SystemData = (WriteStorage<'a, Position>,
                       WriteStorage<'a, Velocity>,
                       ReadStorage<'a, Ball>,
                       ReadStorage<'a, Bar>,
                       Read<'a, resources::GameState>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions,
            mut velocities,
            balls,
            _bars,
            game_state,
        ) = data;

        if !game_state.do_update {
            return;
        }

        for (position, velocity, _) in (&mut positions, &mut velocities, &balls).join() {
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
