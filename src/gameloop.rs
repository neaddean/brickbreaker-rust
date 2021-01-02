use ggez::Context;
use ggez::event::winit_event::*;
use ggez::input::keyboard;
use specs::{RunNow, WorldExt};
use winit::EventsLoop;

use crate::constants::SIMULATION_HZ;
use crate::resources::{EventQueue, GameState};

pub fn run(
    ctx: &mut Context,
    events_loop: &mut EventsLoop,
    dispatcher: &mut specs::Dispatcher,
    world: &mut specs::World,
) {
    {
        let mut game_state = world.write_resource::<GameState>();
        game_state.screen_size = ggez::graphics::drawable_size(ctx);
    }
    loop {
        ctx.timer_context.tick();
        events_loop.poll_events(|event| {
            ctx.process_event(&event);
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        let mut game_state = world.write_resource::<GameState>();
                        game_state.continuing = false;
                    }
                    WindowEvent::ReceivedCharacter(_ch) => {}
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
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
                        input: KeyboardInput {
                            state: ElementState::Released,
                            virtual_keycode: Some(keycode),
                            modifiers,
                            ..
                        },
                        ..
                    } => {
                        {
                            let mut event_queue = world.write_resource::<EventQueue>();
                            event_queue.events.push(crate::events::Event::KeyUp(keycode, modifiers.into()))
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        });

        {
            let mut game_state = world.write_resource::<GameState>();
            while ggez::timer::check_update_time(ctx, SIMULATION_HZ) {
                game_state.pending_updates += 1;
            }
            if !game_state.continuing {
                break;
            }
        }

        dispatcher.dispatch(world);
        world.maintain();
        {
            let mut rs = crate::systems::RenderingSystem { ctx };
            rs.run_now(world);
        }
        ggez::timer::yield_now();
    }
}
