use crate::s;
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Room {
    pub id: String,
    pub desc: String,
}

impl Room {
    pub fn new(id: &str, desc: &str) -> Room {
        Room {
            id: s!(id),
            desc: s!(desc),
        }
    }
}

#[derive(Debug)]
pub struct Rooms<'a> {
    rooms: HashMap<String, Room>,
    phantom: PhantomData<&'a String>,
}

impl<'a> Rooms<'a> {
    pub fn new() -> Rooms<'a> {
        Rooms {
            rooms: HashMap::new(),
            phantom: PhantomData,
        }
    }
    pub fn add(&mut self, room: Room) {
        self.rooms.insert(room.id.clone(), room);
    }

    pub fn find(&self, id: &str) -> Option<&Room> {
        self.rooms.get(&String::from(id))
    }
}
