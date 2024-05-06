use serde::{Serialize, Deserialize};
use crate::devices::shellies::{Shellies, Shelly, State};
use crate::devices::lamps::{Lamps, Lamp};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrontendLamps {
    pub frontend_lamps: Vec<FrontendLamp>
}

impl FrontendLamps {
    pub fn get_frontend_lamps(lamps: &Lamps, shellies: &Shellies) -> Self {
        let mut frontend_lamps = vec![];
        for lamp in lamps.lamps.iter() {
            let (state, brightness) = lamp.read_state(shellies).unwrap();
            let is_on = matches!(state, State::On);
    
            frontend_lamps.push(
                FrontendLamp {
                    id: lamp.id,
                    name: lamp.name.clone(),
                    is_on,
                    brightness,
                }
            )
        }
        FrontendLamps {
            frontend_lamps
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrontendLamp {
    pub id: usize,
    pub name: String,
    pub is_on: bool,
    pub brightness: usize
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrontendConfig {
    pub shellies: Vec<Shelly>,
    pub lamps: Vec<Lamp>
}
