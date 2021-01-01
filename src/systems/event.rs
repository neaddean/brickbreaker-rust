use ggez::input::keyboard::KeyCode;
use rand::{Rng, thread_rng};
use specs::{join::Join, System, Write, WriteStorage};

use crate::{components::*, events::Event, resources::{EntityQueue, EventQueue}};
use crate::entities::EntityType;

pub struct EventSystem;

// System implementation
impl<'a> System<'a> for EventSystem {
    // Data
    type SystemData = (Write<'a, EventQueue>,
                       Write<'a, EntityQueue>,
                       WriteStorage<'a, Velocity>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut event_queue,
            mut entity_queue,
            mut velocities,
        ) = data;

        for event in event_queue.events.drain(..) {
            println!("New event: {:?}", event);

            match event {
                Event::KeyDown(key_code, _key_mods, _is_repeated) => {
                    match (key_code, _is_repeated) {
                        (KeyCode::Up, _) => {
                            for vel in (&mut velocities).join() {
                                vel.x += 2.0 * num::signum(vel.x);
                                vel.y += 2.0 * num::signum(vel.y);
                            }
                        }
                        (KeyCode::Down, _) => {
                            for vel in (&mut velocities).join() {
                                vel.x -= 2.0 * num::signum(vel.x);
                                vel.y -= 2.0 * num::signum(vel.y);
                            }
                        }
                        (KeyCode::Space, false) => {
                            entity_queue.push(EntityType::Ball {
                                x: thread_rng().gen_range(-2.0..2.0),
                                y: thread_rng().gen_range(-2.0..2.0),
                            });
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}