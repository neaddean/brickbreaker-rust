// use specs::{join::Join, System, WriteStorage};
//
// use crate::components::*;
//
// pub struct InputSystem;
//
// // System implementation
// impl<'a> System<'a> for InputSystem {
//     // Data
//     type SystemData = (WriteStorage<'a, Position>, WriteStorage<'a, Velocity>);
//
//     fn run(&mut self, data: Self::SystemData) {
//         events_loop.poll_events(|event| {
//             ctx.process_event(&event);
//             match event {
//                 Event::WindowEvent { event, .. } => match event {
//                     WindowEvent::CloseRequested => {
//                         ctx.continuing = false;
//                     }
//                     _ => {}
//                 },
//                 _ => {}
//             }
//         });
//     }
// }
