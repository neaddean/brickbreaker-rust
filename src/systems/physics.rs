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
            bars,
            game_state,
        ) = data;

        if !game_state.do_update {
            return;
        }

        // move bar and check if it's at the edge of screen
        for (position, velocity, bar) in (&mut positions, &mut velocities, &bars).join() {
            position.x += velocity.x;
            position.y += velocity.y;

            if position.x < bar.width / 2.0 {
                position.x = bar.width / 2.0;
            } else if position.x + bar.width / 2.0 > game_state.screen_size.0 {
                position.x = game_state.screen_size.0 - bar.width / 2.0;
            }
        }

        // move alls and if they are colliding with anything, reverse velocity
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
