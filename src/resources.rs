#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<crate::events::Event>,
}

// #[derive(Default)]
// pub struct EntityQueue {
//     pub entites: Vec<crate::entities::EntityType>,
// }

pub type EntityQueue = Vec<crate::entities::EntityType>;

#[derive(Default)]
pub struct GameTime {
    pub do_update : bool,
}
