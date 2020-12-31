use ggez::input::keyboard::{KeyCode};
use specs::{join::Join, System, Write, WriteStorage};

use crate::{components::*, events::Event, resources::EventQueue};

pub struct EventSystem;

// System implementation
impl<'a> System<'a> for EventSystem {
    // Data
    type SystemData = (Write<'a, EventQueue>,
                       WriteStorage<'a, Velocity>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut event_queue,
            mut velocities,
        ) = data;

        for event in event_queue.events.drain(..) {
            println!("New event: {:?}", event);

            match event {
                Event::KeyDown(KeyCode, KeyMods, is_repeated) => {
                    match KeyCode {
                        KeyCode::Up => {
                            for vel in (&mut velocities).join() {
                                vel.x += 2.0;
                                vel.y += 2.0;
                            }
                        }
                        KeyCode::Down => {
                            for vel in (&mut velocities).join() {
                                vel.x -= 2.0;
                                vel.y -= 2.0;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}