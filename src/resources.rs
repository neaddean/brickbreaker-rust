#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<crate::events::Event>,
}

#[derive(Default)]
pub struct EntityQueue {
    pub entites: Vec<crate::entities::EntityType>,
}