use ggez::input::keyboard::KeyCode;
use rand::{Rng, thread_rng};
use specs::{join::Join, ReadStorage, System, Write, WriteStorage};

use crate::{components::*, events::Event, resources::{EntityQueue, EventQueue}};
use crate::entities::EntityType;

pub struct EventSystem;

// System implementation
impl<'a> System<'a> for EventSystem {
    // Data
    type SystemData = (Write<'a, EventQueue>,
                       Write<'a, EntityQueue>,
                       WriteStorage<'a, Position>,
                       WriteStorage<'a, Velocity>,
                       ReadStorage<'a, Ball>,
                       ReadStorage<'a, Bar>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut event_queue,
            mut entity_queue,
            _positions,
            mut velocities,
            balls,
            bars,
        ) = data;

        for event in event_queue.events.drain(..) {
            println!("New event: {:?}", event);
            match event {
                Event::KeyDown(key_code, _key_mods, _is_repeated) => {
                    match (key_code, _is_repeated) {
                        (KeyCode::Up, _) => {
                            for (vel, _) in (&mut velocities, &balls).join() {
                                vel.x += 2.0 * num::signum(vel.x);
                                vel.y += 2.0 * num::signum(vel.y);
                            }
                        }
                        (KeyCode::Down, _) => {
                            for (vel, _) in (&mut velocities, &balls).join() {
                                vel.x -= 2.0 * num::signum(vel.x);
                                vel.y -= 2.0 * num::signum(vel.y);
                            }
                        }
                        (KeyCode::Right, false) => {
                            for (vel, _) in (&mut velocities, &bars).join() {
                                vel.x += 10.0;
                            }
                        }
                        (KeyCode::Left, false) => {
                            for (vel, _) in (&mut velocities, &bars).join() {
                                vel.x -= 10.0;
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
                Event::KeyUp(key_code, _key_mods) => {
                    match key_code {
                        KeyCode::Right => {
                            for (vel, _) in (&mut velocities, &bars).join() {
                                vel.x -= 10.0;
                            }
                        }
                        KeyCode::Left => {
                            for (vel, _) in (&mut velocities, &bars).join() {
                                vel.x += 10.0;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}