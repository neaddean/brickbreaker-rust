use std::cell::RefCell;
use std::rc::Rc;

use ggez::Context;
use ggez::event::winit_event::*;
use ggez::input::keyboard;
use specs::{System, Write};
use winit::EventsLoop;

use crate::events::Event::{CloseGame, KeyDown, KeyUp};
use crate::resources::{EventQueue};

pub struct InputSystem<'a> {
    pub ctx: Rc<RefCell<&'a mut Context>>,
    pub event_loop: &'a mut EventsLoop,
}

impl<'a> System<'a> for InputSystem<'_> {
    type SystemData = Write<'a, EventQueue>;

    fn run(&mut self, data: Self::SystemData) {
        let mut event_queue = data;
        let ref mut ctx = self.ctx.borrow_mut();

        self.event_loop.poll_events(|event| {
            ctx.process_event(&event);
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::ReceivedCharacter(_ch) => {}
                    WindowEvent::CloseRequested => {
                        event_queue.events.push(CloseGame)
                    }
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
                        event_queue.events.push(KeyDown(keycode, modifiers.into(), repeat))
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
                        event_queue.events.push(KeyUp(keycode, modifiers.into()))
                    }
                    _ => {}
                },
                _ => {}
            }
        });
    }
}