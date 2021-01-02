use specs::{Entities, join::Join, ReadStorage, System, WriteStorage, ReadExpect};

use crate::components::*;
use crate::constants::SIMULATION_DURATION;
use crate::resources;

#[derive(Default)]
pub struct PhysicsSystem {
    accum: f32
}

struct BarDescriptor {
    pub x: f32,
    pub y: f32,
    pub height: f32,
    pub width: f32,
}

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (WriteStorage<'a, Position>,
                       WriteStorage<'a, Velocity>,
                       ReadStorage<'a, Ball>,
                       ReadStorage<'a, Bar>,
                       ReadExpect<'a, resources::GameState>,
                       Entities<'a>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions,
            mut velocities,
            balls,
            bars,
            game_state,
            entities
        ) = data;

        self.accum += game_state.this_duration().as_secs_f32();
        while self.accum > SIMULATION_DURATION {
            self.accum -= SIMULATION_DURATION;
            let mut bar_desc: Vec<BarDescriptor> = Vec::new();

            // move bar and check if it's at the edge of screen
            for (position, velocity, bar) in
            (&mut positions, &mut velocities, &bars).join() {
                position.x += velocity.x * SIMULATION_DURATION;
                position.y += velocity.y * SIMULATION_DURATION;

                if position.x < bar.width / 2.0 {
                    position.x = bar.width / 2.0;
                } else if position.x + bar.width / 2.0 > game_state.screen_size.0 {
                    position.x = game_state.screen_size.0 - bar.width / 2.0;
                }
                bar_desc.push(BarDescriptor {
                    x: position.x,
                    y: position.y,
                    height: bar.height,
                    width: bar.width,
                });
            }

            // move balls and if they are colliding with anything, reverse velocity
            for (entity, position, velocity, ball) in
            (&entities, &mut positions, &mut velocities, &balls).join() {
                position.x += velocity.x * SIMULATION_DURATION;
                position.y += velocity.y * SIMULATION_DURATION;

                if position.x + ball.radius / 2.0 > game_state.screen_size.0 {
                    position.x = game_state.screen_size.0 - ball.radius / 2.0;
                    velocity.x *= -1.0;
                } else if position.x < ball.radius / 2.0 {
                    position.x = ball.radius / 2.0;
                    velocity.x *= -1.0;
                }

                if position.y > game_state.screen_size.1 {
                    entities.delete(entity).unwrap();
                } else if position.y < 0.0 {
                    position.y = 0.0;
                    velocity.y *= -1.0;
                }

                for bar in &bar_desc {
                    if (position.y + ball.radius / 2.0 > bar.y - bar.height / 2.0) &
                        (position.x < bar.x + bar.width / 2.0) &
                        (position.x > bar.x - bar.width / 2.0)
                    {
                        velocity.y *= -1.0;
                        position.y = bar.y - bar.height / 2.0 - ball.radius / 2.0;
                        // let offset = position.x - bar.x - bar.width / 2.0;
                        // position.y = bar.y - bar.height / 2.0 - ball.radius / 2.0;
                        // velocity.x = 1.0 + 6.0 * offset / bar.width / 2.0;
                        // velocity.y *= -6.0 + offset * 2.0 / bar.width * 4.0
                    }
                }
            }
        }
    }
}