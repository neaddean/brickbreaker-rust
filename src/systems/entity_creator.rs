use specs::{Entities, System, Write, WriteStorage};

use crate::{components::*, entities::EntityType, resources::EntityQueue};

pub struct EntityCreatorSystem;


// System implementation
impl<'a> System<'a> for EntityCreatorSystem {
    // Data
    type SystemData = (Write<'a, EntityQueue>,
                       Entities<'a>,
                       WriteStorage<'a, Velocity>,
                       WriteStorage<'a, Position>,
                       WriteStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut entity_queue,
            entites,
            mut velocities,
            mut positions,
            mut renderables
        ) = data;

        for entity_to_create in entity_queue.drain(..) {
            match entity_to_create {
                EntityType::Ball { x, y } => {
                    entites.build_entity()
                        .with(Position {
                            x: 0.0,
                            y: 0.0,
                            z: 0,
                        }, &mut positions)
                        .with(Velocity {
                            x,
                            y,
                        }, &mut velocities)
                        .with(Renderable {
                            texture: "/ball.png".to_string()
                        }, &mut renderables)
                        .build();
                }
                _ => {}
            }
        }
    }
}