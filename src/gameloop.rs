use ggez::event::winit_event::*;
use ggez::Context;
use winit::EventsLoop;
use specs::RunNow;


pub fn run(
    ctx: &mut Context,
    events_loop: &mut EventsLoop,
    dispatcher: &mut specs::Dispatcher,
    world: &mut specs::World,
) {
    // use ggwz::input::{keyboard, mouse};

    while ctx.continuing {
        // If you are writing your own event loop, make sure
        // you include `timer_context.tick()` and
        // `ctx.process_event()` calls.  These update ggez's
        // internal state however necessary.
        ctx.timer_context.tick();
        events_loop.poll_events(|event| {
            ctx.process_event(&event);
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        ctx.continuing = false;
                    }
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
