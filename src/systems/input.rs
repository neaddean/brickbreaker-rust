use std::cell::RefCell;
use std::rc::Rc;

use ggez::Context;
use ggez::event::winit_event::*;
use ggez::input::{keyboard, mouse};
use specs::{System, Write};
use winit::{dpi, EventsLoop};

use crate::events::Event::{CloseGame, KeyDown, KeyUp};
use crate::ImGuiWrapper;
use crate::resources::EventQueue;

pub struct InputSystem<'a> {
    ctx: Rc<RefCell<&'a mut Context>>,
    imgui_wrapper: Rc<RefCell<&'a mut ImGuiWrapper>>,
    event_loop: &'a mut EventsLoop,
}

impl<'a> InputSystem<'a> {
    pub fn new(ctx: Rc<RefCell<&'a mut Context>>,
               imgui_wrapper: Rc<RefCell<&'a mut ImGuiWrapper>>,
               event_loop: &'a mut EventsLoop) -> Self {
        InputSystem {
            ctx,
            imgui_wrapper,
            event_loop,
        }
    }
}

impl<'a> System<'a> for InputSystem<'_> {
    type SystemData = Write<'a, EventQueue>;

    fn run(&mut self, data: Self::SystemData) {
        let mut event_queue = data;
        let ref mut ctx = self.ctx.borrow_mut();
        let ref mut imgui_wrapper = self.imgui_wrapper.borrow_mut();

        self.event_loop.poll_events(|event| {
            ctx.process_event(&event);
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::ReceivedCharacter(ch) => {
                        imgui_wrapper.update_text(ch);
                    }
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
                        let keymods = modifiers.into();
                        let repeat = keyboard::is_key_repeated(ctx);
                        event_queue.events.push(KeyDown(keycode, keymods, repeat));
                        imgui_wrapper.update_key_down(keycode, keymods);
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
                        let keymods = modifiers.into();
                        event_queue.events.push(KeyUp(keycode, keymods));
                        imgui_wrapper.update_key_up(keycode, keymods);
                    }
                    WindowEvent::Resized(..) => {}
                    WindowEvent::MouseWheel { delta, .. } => {
                        let (x, y) = match delta {
                            MouseScrollDelta::LineDelta(x, y) => (x, y),
                            MouseScrollDelta::PixelDelta(dpi::LogicalPosition { x, y }) => {
                                (x as f32, y as f32)
                            }
                        };
                        imgui_wrapper.update_scroll(x, y);
                    }
                    WindowEvent::MouseInput {
                        state: element_state,
                        button,
                        ..
                    } => {
                        match element_state {
                            ElementState::Pressed => {
                                imgui_wrapper.update_mouse_down(button);
                            }
                            ElementState::Released => {
                                imgui_wrapper.update_mouse_up(button);
                            }
                        }
                    }
                    WindowEvent::CursorMoved { .. } => {
                        let position = mouse::position(ctx);
                        let _delta = mouse::delta(ctx);
                        imgui_wrapper.update_mouse_pos(position.x, position.y);
                    }
                    _x => {
                        // trace!("ignoring window event {:?}", _x);
                    }
                },
                Event::DeviceEvent { event, .. } => match event {
                    _ => (),
                },
                Event::Awakened => (),
                Event::Suspended(_) => (),
            }
        });
    }
}
