use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Room {
    pub number: u32,
    pub beds: u8,
    pub price: f32,
    pub occupied: bool,
}

#[allow(dead_code)]
pub struct Hotel {
    rooms: HashMap<u32, Room>,
}

#[allow(dead_code)]
impl Hotel {
    pub fn new() -> Self {
        Hotel {
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room.number, room);
    }

    pub fn check_in(&mut self, room_number: u32) -> Result<(), String> {
        match self.rooms.get_mut(&room_number) {
            Some(room) => {
                if room.occupied {
                    Err("Room already occupied".to_string())
                } else {
                    room.occupied = true;
                    Ok(())
                }
            }
            None => Err("Room not found".to_string()),
        }
    }

    pub fn check_out(&mut self, room_number: u32) -> Result<(), String> {
        match self.rooms.get_mut(&room_number) {
            Some(room) => {
                if !room.occupied {
                    Err("Room already vacant".to_string())
                } else {
                    room.occupied = false;
                    Ok(())
                }
            }
            None => Err("Room not found".to_string()),
        }
    }

    pub fn get_available_rooms(&self) -> Vec<&Room> {
        self.rooms.values().filter(|room| !room.occupied).collect()
    }


    pub fn get_room_info(&self, room_number: u32) -> Option<&Room> {
         self.rooms.get(&room_number)
    }

}