use specs::{Entities, Read, System, Write, WriteStorage};

use crate::{components::*, entities::EntityType, resources::{EntityQueue, GameState}};

pub struct EntityCreatorSystem;


// System implementation
impl<'a> System<'a> for EntityCreatorSystem {
    // Data
    type SystemData = (Write<'a, EntityQueue>,
                       Entities<'a>,
                       WriteStorage<'a, Velocity>,
                       WriteStorage<'a, Position>,
                       WriteStorage<'a, Renderable>,
                       WriteStorage<'a, Ball>,
                       WriteStorage<'a, Bar>,
                       Read<'a, GameState>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut entity_queue,
            entites,
            mut velocities,
            mut positions,
            mut renderables,
            mut ball_storage,
            mut bar_storage,
            game_state,
        ) = data;

        for entity_to_create in entity_queue.drain(..) {
            match entity_to_create {
                EntityType::Ball { x, y } => {
                    entites.build_entity()
                        .with(Ball,
                              &mut ball_storage)
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
                            asset_name: "/ball.png".to_string()
                        }, &mut renderables)
                        .build();
                }
                EntityType::Bar => {
                    entites.build_entity()
                        .with(Bar,
                              &mut bar_storage)
                        .with(Position {
                            x: game_state.screen_size.0 / 2.0,
                            y: game_state.screen_size.1 / 2.0,
                            z: 0,
                        }, &mut positions)
                        // .with(Velocity {
                        //     0.0,
                        //     0.0,
                        // }, &mut velocities)
                        .with(Renderable {
                            asset_name: "/bar.png".to_string()
                        }, &mut renderables)
                        .build();
                }
                _ => {}
            }
        }
    }
}