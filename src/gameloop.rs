use ggez::Context;
use ggez::event::winit_event::*;
use ggez::input::keyboard;
use specs::{RunNow, WorldExt};
use winit::EventsLoop;

use crate::resources::EventQueue;

pub fn run(
    ctx: &mut Context,
    events_loop: &mut EventsLoop,
    dispatcher: &mut specs::Dispatcher,
    world: &mut specs::World,
) {
    while ctx.continuing {
        ctx.timer_context.tick();
        events_loop.poll_events(|event| {
            ctx.process_event(&event);
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        ctx.continuing = false;
                    }
                    WindowEvent::ReceivedCharacter(_ch) => {}
                    WindowEvent::KeyboardInput {
                        input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(keycode),
                            modifiers,
                            ..
                        },
                        ..
                    } => {
                        let repeat = keyboard::is_key_repeated(ctx);
                        {
                            let mut event_queue = world.write_resource::<EventQueue>();
                            event_queue.events.push(crate::events::Event::KeyDown(keycode, modifiers.into(), repeat))
                        }
                    }
                    WindowEvent::KeyboardInput {
                        input:
                        KeyboardInput {
                            state: ElementState::Released,
                            virtual_keycode: Some(keycode),
                            modifiers,
                            ..
                        },
                        ..
                    } => {}
                    _ => {}
                },
                _ => {}
            }
        });
        dispatcher.dispatch(world);
        {
            let mut rs = crate::systems::RenderingSystem { ctx };
            rs.run_now(world);
        }
    }
}
