use crate::events::Event;

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>,
}