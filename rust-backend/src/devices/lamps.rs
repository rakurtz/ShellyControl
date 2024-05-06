use serde::{Deserialize, Serialize};
use crate::devices::shellies::{Shellies, State};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lamps {
    pub lamps: Vec<Lamp>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lamp {
    pub id: usize,
    pub name: String,
    pub members: Vec<Member>
}  

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Member {
    pub device_id: usize,
    pub lane: usize,
}

impl Lamps {
    pub async fn global_off(&mut self, shellies: &mut Shellies) -> Result<(), reqwest::Error>{
        for lamp in self.lamps.iter_mut() {
            lamp.switch(&State::Off, shellies).await?;
        }
        Ok(())
    }

    pub async fn global_brightness(&mut self, brightness: usize, shellies: &mut Shellies) -> Result<(), reqwest::Error> {
        
        for lamp in self.lamps.iter_mut() {
            lamp.set_brightness(brightness, shellies).await?;
        }
        Ok(())
    }
    pub async fn switch_single(&mut self, id: usize, state: &State, shellies: &mut Shellies) -> Result<(), String> {
        let lamp = self.get_mut_lamp_by_id(id)?;
        if let Err(message) = lamp.switch(state, shellies).await {
            return Err(message.to_string());
        }
        Ok(())
    }

    pub async fn set_brightness_single(&mut self, id: usize, brightness: usize, shellies: &mut Shellies) -> Result<(), String> {
        let lamp = self.get_mut_lamp_by_id(id)?;
        if let Err(message) = lamp.set_brightness(brightness, shellies).await {
            return Err(message.to_string());
        }
        Ok(())
    }

    pub fn get_mut_lamp_by_id(&mut self, id: usize) -> Result<&mut Lamp, String> {
        for lamp in self.lamps.iter_mut() {
            if lamp.id == id {
                return Ok(lamp);
            }
        }
        Err(format!("Error: Lamp not found: {}", id))
    }

    pub fn get_lamp_by_id(&self, id: usize) -> Result<&Lamp, String> {
        for lamp in self.lamps.iter() {
            if lamp.id == id {
                return Ok(lamp);
            }
        }
        Err(format!("Error: Lamp not found: {}", id))
    }

    pub fn add_lamp(&mut self, mut received_lamp: Lamp) -> usize {
        let new_id = self.new_id();
        received_lamp.id = new_id;
        self.lamps.push(received_lamp);
        new_id
    }

    pub fn delete_lamp_by_id(&mut self, id: usize) -> Result<(), String> {
        if self.get_lamp_by_id(id).is_err() {
            return Err(format!("Couldn't find Lamp with id {}", id));
        }
        let index = self.lamps.iter().enumerate().find(|(_, lamp)| lamp.id == id);
        match index {
            Some((idx, _)) => self.lamps.remove(idx),
            None => return Err(format!("Lamp with given id not found.")),
        };

        Ok(())
    }

    pub fn modify_lamp_by_id(&mut self, received_lamp: Lamp) -> Result<(), String> {
        match self.get_mut_lamp_by_id(received_lamp.id) {
            Err(_) => return Err(format!("Couldn't find Lamp with id {}", received_lamp.id)),
            Ok(lamp) => {
                lamp.name = received_lamp.name.clone();
                lamp.members = received_lamp.members.clone();
            },
        };
        
        Ok(())
    }

    fn new_id(&self) -> usize {
        let mut id = 0;
        for lamp in &self.lamps {
            if lamp.id > id {
                id = lamp.id;
            }
        }
        id += 1;
        id
    } 

}


impl Lamp {
    pub async fn switch(&mut self, command: &State, shellies: &mut Shellies) -> Result<(State, usize), reqwest::Error> {
        for member in self.members.iter_mut() {
            if let Ok(shelly) = shellies.get_mut_shelly_by_id(member.device_id) {
                shelly.set_state(member.lane, command).await?;
            }
        }
        let (state, brightness) = self.read_state(shellies).unwrap();
        Ok((*state, brightness))
    }
    
    pub async fn set_brightness(&self, brightness: usize, shellies: &mut Shellies) -> Result<(), reqwest::Error> {
        for member in self.members.iter() {
            if let Ok(shelly) = shellies.get_mut_shelly_by_id(member.device_id) {
                shelly.set_brightness(member.lane, brightness).await?
            }
        }
        Ok(())
    }
    pub fn read_state<'a>(&self, shellies: &'a Shellies) -> Result<(&'a State, usize), String> {
        match shellies.get_shelly_by_id(self.members[0].device_id) {
            Err(e) => Err(e),
            Ok(shelly) => {
                let state = &shelly.lights[self.members[0].lane].state;
                let brightness = shelly.lights[self.members[0].lane].brightness;
                Ok((state, brightness))
            }
        }
    }
}