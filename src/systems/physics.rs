use specs::{    join::Join, System, WriteStorage};

use crate::components::*;

pub struct PhysicsSystem;

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
