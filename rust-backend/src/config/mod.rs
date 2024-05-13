use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use crate::devices::lamps::{Lamp, Lamps};
use crate::devices::shellies::{Shellies, Shelly};

// ConfigShelly
// using a reduced shelly-struct here to (de-)serialize to/from .yaml config file
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigShelly {
    pub id: usize,
    pub ip: String,
    pub device_name: String, 
}

impl ConfigShelly {
    // when writing programm state to .yaml file we need to reduce full blown Shelly to ConfigShelly
    pub fn from_shelly(shelly: &Shelly) -> Self {
        Self {
            id: shelly.id,
            ip: shelly.ip.clone(),
            device_name: shelly.device_name.clone()
        }
    }
}

// FromConfig
// the actual config as a struct to be serializable to the yaml config file
#[derive(Debug, Serialize, Deserialize)]
pub struct FromConfig {
    pub shellies: Vec<ConfigShelly>,
    pub lamps: Vec<Lamp>,
}

impl FromConfig {
    pub fn read_config_shellies() -> Result<Self, Box<dyn std::error::Error>> {
        let path = std::env::var("CONFIG_FILE")
            .expect("Error in environment variables: CONFIG_FILE not found (check compose.yaml)");
    
        let f = OpenOptions::new()
            .read(true)
            .open(path)?;
        let config: Self = serde_yaml::from_reader(f)?;
        Ok(config)
    }

    pub fn new_from_state(shellies: Vec<Shelly>, lamps: Vec<Lamp>) -> Self {
        let mut config_shellies = vec![];
        for shelly in &shellies {
            config_shellies.push(
                ConfigShelly::from_shelly(shelly)
            );
        }
        Self {
            shellies: config_shellies,
            lamps: lamps.clone()
        }
    }

    pub fn write_to_yaml(&self) -> Result<(), Box<dyn std::error::Error>> {

        let path = std::env::var("CONFIG_FILE")
        .expect("Error in environment variables: CONFIG_FILE not found (check compose.yaml)");

        let f = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;
        serde_yaml::to_writer(f, &self)?;
        Ok(())
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn read_from_yaml() {
        // load .env
        dotenvy::dotenv().ok();
        let config = FromConfig::read_config_shellies().expect("could not read from yaml file");
        assert_eq!(config.shellies.len(), 6); 
        assert_eq!(config.lamps.len(), 7);
    } 

    #[test]
    fn write_to_yaml() {
        // load .env
        dotenvy::dotenv().ok();
        let config = FromConfig::read_config_shellies().expect("could not read from yaml file");
        let result = config.write_to_yaml();
        assert!(result.is_ok());
    } 
}