use serde::{Deserialize, Serialize};
use reqwest::Client;
use crate::config::ConfigShelly;


#[derive(Serialize, Deserialize, Debug)]
pub struct Shellies {
    pub shellies: Vec<Shelly>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shelly {
    pub id: usize,
    pub device_name: String,
    pub ip: String,
    pub lights: Vec<Light>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Light {
    pub state: State,
    pub brightness: usize,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    On,
    Off,
    Toggle,
    Unknown
}

#[derive(Deserialize)]
struct ShellyResponse {
    #[serde(rename(deserialize = "ison"))]
    is_on: bool,
    brightness: usize,
}

impl Shellies {
    pub async fn new_from_config_shellies(config_shellies: Vec<ConfigShelly>) -> Self {
        let mut shellies = vec![];
        for config_shelly in config_shellies {
            shellies.push(Shelly::new_by_id_name_ip(config_shelly.id, config_shelly.device_name, config_shelly.ip));
            
        }
        let mut shellies = Shellies {
            shellies
        };
        let _ = shellies.get_states().await;
        shellies
    }

    pub fn add_from_config_shelly(&mut self, config_shelly: ConfigShelly) -> usize {
        let new_id = self.new_id();
        let shelly = Shelly::new_by_id_name_ip(new_id, config_shelly.device_name, config_shelly.ip);
        self.shellies.push(shelly);
        new_id
    }

    pub fn delete_shelly_by_id(&mut self, id: usize) -> Result<(), String> {
        if self.get_shelly_by_id(id).is_err() {
            return Err(format!("Couldn't find Shelly with id {}", id));
        }
        let index = self.shellies.iter().enumerate().find(|(_, shelly)| shelly.id == id);
        match index {
            Some((idx, _)) => self.shellies.remove(idx),
            None => return Err(format!("Shelly with given id not found.")),
        };

        Ok(())
    }

    pub fn modify_shelly_by_id(&mut self, received_shelly: ConfigShelly) -> Result<(), String> {
        match self.get_mut_shelly_by_id(received_shelly.id) {
            Err(_) => return Err(format!("Couldn't find Shelly with id {}", received_shelly.id)),
            Ok(shelly) => {
                shelly.device_name = received_shelly.device_name.clone();
                shelly.ip = received_shelly.ip.clone();
            },
        };
        
        Ok(())
    }

    pub async fn get_states(&mut self) -> Result<(), reqwest::Error> {
        // using drain to actually work on owned elements of the vector
        let mut set = tokio::task::JoinSet::new();

        for mut shelly in self.shellies.drain(..) {
            set.spawn(async move {
                let result = shelly.get_state().await;
                // returning shelly to add to self.shellies later
                (shelly, result)
            });
        }
        
        //
        // self.shellies is empty here!
        //
        
        while let Some(Ok((shelly, result))) = set.join_next().await {
            self.shellies.push(shelly);  // Changes order of the shellies in the `Vec`
            
            if let Err(e) = result {
                eprintln!("Error: {}", e);
            }
        }
        
        Ok(())
    }

    // pub async fn toggle_all(&mut self) -> Result<(), reqwest::Error> {
    //     if self.shellies.iter()
    //         .filter(|shelly| shelly.lights.iter().filter(|light| light.state == State::On).count() > 0).count() == 0 {
    //             self.global_switch(&State::On).await?;
    //     } else {
    //         self.global_switch(&State::Off).await?;
    //     }
    //     Ok(())
    // }

    pub async fn global_switch(&mut self, command: &State) -> Result<(), reqwest::Error> {

        // TODO: implement with join_set!
        for shelly in self.shellies.iter_mut() {
            for i in 0..shelly.lights.len() {
                println!("switching {:?} lane {} at {}", command, i, shelly.device_name);
                shelly.set_state(i, command).await?;
            }
        }
        Ok(())
    }


    pub fn get_mut_shelly_by_id(&mut self, id: usize) -> Result<&mut Shelly, String> {
        for shelly in self.shellies.iter_mut() {
            if shelly.id == id {
                return Ok(shelly);
            }
        }
        Err(format!("couldn't find shelly by id {}", id))
    }

    pub fn get_shelly_by_id(&self, id: usize) -> Result<&Shelly, String> {
        for shelly in self.shellies.iter() {
            if shelly.id == id {
                return Ok(shelly);
            }
        }
        Err(format!("couldn't find shelly by id {}", id))
    }

    fn new_id(&self) -> usize {
        let mut id = 0;
        for shelly in &self.shellies {
            if shelly.id > id {
                id = shelly.id;
            }
        }
        id += 1;
        id
    }

}

impl Shelly {
    pub fn new_by_id_name_ip(id: usize, name: String, ip: String) -> Self {
        let mut lights = vec![];

        for _ in 0..4 {
            lights.push(
                Light {
                    state: State::Unknown,
                    brightness: 50,
                }
            );
        }
        Shelly {
            id,
            device_name: name,
            ip,
            lights
        }
    }
    pub async fn get_state(&mut self) -> Result<(), reqwest::Error> {
        let mut handles = Vec::with_capacity(4);
        let req = Client::new();

        for (idx, mut light) in self.lights.drain(..).enumerate() {
            let req = req.clone();
            let ip = self.ip.clone();
        
            let handle = tokio::spawn(async move {
                let url = format!("http://{}/white/{}", ip, idx);
                match req.get(&url).send().await {
                    Err(e) => eprint!("Error in connecting to shelly at {}.\n{:?}", url, e),
                    Ok(response) => {
                        match response.json::<ShellyResponse>().await {
                            Err(e) => eprint!("Failed parsing json: {:?}", e),
                            Ok(shelly) => {
                                if shelly.is_on {
                                    light.state = State::On;
                                } else {
                                    light.state = State::Off;
                                } 
                                light.brightness = shelly.brightness;
                            },
                        } 
                    },
                }
                light
            });
            handles.push(handle);
        }
        

        for handle in handles {
            match handle.await {
                Ok(light) => self.lights.push(light),
                Err(e) => eprintln!("Error: Join_Error {:?}", e)
            }
        }

        Ok(())

    }

    pub async fn set_state(&mut self, lane: usize, command: &State) -> Result<(), reqwest::Error> {
        let suffix = match command {
            State::On => "on",
            State::Off => "off",
            State::Toggle => "toggle",
            _ => panic!("Wrong usage of internal function. Command not found."),
        };

        let url = format!("http://{}/white/{}?turn={}", self.ip, lane, suffix);
        let req = Client::new();
        let response = req.get(url).send().await?;
        let result: ShellyResponse = response.json().await?;
        if result.is_on {
            self.lights[lane].state = State::On;
        } else {
            self.lights[lane].state = State::Off;
        }
        Ok(())
    }

    pub async fn set_brightness(&mut self, lane: usize, brightness: usize) -> Result<(), reqwest::Error> {
        let url = format!("http://{}/white/{}?brightness={}", self.ip, lane, brightness);
        let req = Client::new();
        let response = req.get(url).send().await?;
        let result: ShellyResponse = response.json().await?;
    
        self.lights[lane].brightness = result.brightness;
        
        Ok(())
    }

}




