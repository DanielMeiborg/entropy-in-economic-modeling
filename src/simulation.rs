use uuid::Uuid;

#[derive(PartialEq)]
pub enum Capacity {
    Tag,
    Count(u64),
}

#[derive(PartialEq)]
pub struct Entity {
    pub name: String,
    pub resources: Vec<Capacity>,
}

pub struct Resource {
    pub name: String,
    pub capacity: Capacity,
    pub limited: bool,
    pub must_be_held: bool,
    pub can_hold_multiple: bool,
}

#[derive(PartialEq)]
pub struct State {
    pub entities: Vec<Entity>,
}

pub struct Simulation {
    pub resources: Vec<Resource>,
    pub initial_state: Box<State>,
    pub current_state: Box<State>,
    pub current_time: u64,
}
