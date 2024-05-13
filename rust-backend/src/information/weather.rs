
use openweather_sdk::{responses::{CurrentResponse, ForecastResponse}, Language, OpenWeather, Units};
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrontendWeather {
    pub current: FrontendWeatherCurrent,
    pub forecast: FrontendWeatherForecast,
}

impl FrontendWeather {
    fn open_weather_icon_url(icon: Option<&str>) -> String {
        if let Some(icon) = icon {
            format!("https://openweathermap.org/img/wn/{}@2x.png", icon)
        } else {
            // just chose an icon to have valid url in frontend
            "https://openweathermap.org/img/wn/50d@2x.png".to_string()
        }
    }

    pub fn new() -> Self {
        Self {
            current: FrontendWeatherCurrent::empty(),
            forecast: FrontendWeatherForecast::empty()
        }
    }


    pub async fn update(&mut self) -> Result<()> {
        let mut weather_sdk = WeatherSDK::new()?; 
        weather_sdk.update().await?;
        println!("After update()");
        self.current = FrontendWeatherCurrent::new(&weather_sdk.current.unwrap());
        self.forecast = FrontendWeatherForecast::new(&weather_sdk.forecast.unwrap());
        Ok(())
    }


}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrontendWeatherCurrent {
    pub temp: f64,
    pub feels_like: f64,
    pub description: String,
    pub icon_url: String,
    pub wind_speed: f64,
}

impl FrontendWeatherCurrent {
    fn new(current: &CurrentResponse) -> Self {
        let current_weather = current.weather.first().unwrap(); // todo: check this unwrap()!
        Self {
            temp: current.main.temp,
            feels_like: current.main.feels_like,
            description: current_weather.description.clone(),
            icon_url: FrontendWeather::open_weather_icon_url(Some(&current_weather.icon)),
            wind_speed: current.wind.speed,
        }
    }

    fn empty() -> Self {
        Self {
            temp: 42.0,
            feels_like: 66.0,
            description: "Could not retrieve current weather information".into(),
            icon_url: FrontendWeather::open_weather_icon_url(None),
            wind_speed: 0.0, // 
        }
    }


}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForecastItem {
    datetime: u64,
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    description: String,
    icon: String,
    wind_speed: f64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrontendWeatherForecast {
    forecast: Vec<ForecastItem>
}    

impl FrontendWeatherForecast {
    pub fn empty() -> Self {
        Self {
            forecast: vec![],
        }    
    }

    pub fn new(forecast_response: &ForecastResponse) -> Self {
        let mut forecast = vec![];

        for item in forecast_response.list.iter() {
            forecast.push(
                ForecastItem {
                    datetime: item.datetime,
                    temp: item.main.temp,
                    feels_like: item.main.feels_like,
                    temp_min: item.main.temp_min,
                    temp_max: item.main.temp_max,
                    description: item.weather.first().unwrap().description.clone(),
                    icon: FrontendWeather::open_weather_icon_url(Some(&item.weather.first().unwrap().icon)),
                    wind_speed: item.wind.speed,
                }
        );

        }

        FrontendWeatherForecast {
            forecast
        }
    }
}    





#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeatherSDK {
    openweather: OpenWeather,
    lat: f64,
    lon: f64,
    forecast_count: u8,
    pub current: Option<CurrentResponse>,
    pub forecast: Option<ForecastResponse>,
}

impl WeatherSDK {
    pub fn new() -> Result<Self> {
        let openweather = OpenWeather::new(
            std::env::var("OPENWEATHER_API_KEY")?.to_string(),
            Units::Metric,
            Language::German
        );
        
        let lat = std::env::var("OPENWEATHER_LAT")?
                .parse::<f64>()?;

        let lon = std::env::var("OPENWEATHER_LON")?
                .parse::<f64>()?;
        
        let forecast_count = std::env::var("OPENWEATHER_FORECAST_COUNT")?
        .parse::<u8>()?;
        Ok(WeatherSDK { openweather, lat, lon, forecast_count, current: None, forecast: None})
    }

    pub async fn retrieve_forecast(&self) -> Result<ForecastResponse> {
        match self.openweather.forecast.call(self.lat, self.lon, self.forecast_count).await {
            Ok(forecast) => Ok(forecast),
            Err(e) => Err(anyhow!(e.as_ref().to_string())), // used to "convert" from dyn Error to anyhow to be "Send"
        }
        
    }   

    pub async fn retrieve_current(&self) -> Result<CurrentResponse> {
        match self.openweather.current.call(self.lat, self.lon).await {
            Ok(current) => Ok(current),
            Err(e) => Err(anyhow!(e.as_ref().to_string())), // used to "convert" from dyn Error to anyhow to be "Send"
        }
        
    } 

    pub async fn update(&mut self) -> Result<()>{ 
        let (current, forecast) = tokio::join!(
                                                                        self.retrieve_current(), 
                                                                        self.retrieve_forecast()
                                                                    );
        
        self.current = Some(current?);
        self.forecast = Some(forecast?);
        Ok(())
    }

}






// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;

    fn _get_env() {
        dotenv().ok();
    }


    #[tokio::test]
    async fn open_weather_sdk() {
        _get_env();
        let mut weather = WeatherSDK::new().expect("Couldn't Weather::new()");
        let _= weather.update().await;

        // retrieved information
        assert!(weather.current.is_some());
        assert!(weather.forecast.is_some());
    }


    #[tokio::test]
    async fn weather_frontends() {
        _get_env();
        let mut frontend_weather = FrontendWeather::new();
        frontend_weather.update().await.unwrap();

        let count = std::env::var("OPENWEATHER_FORECAST_COUNT").unwrap().parse::<u8>().unwrap();

        assert_eq!(frontend_weather.forecast.forecast.len(), count as usize)
    }

}