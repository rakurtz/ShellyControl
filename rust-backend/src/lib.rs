pub mod routes;
pub mod devices;
pub mod information;
pub mod config;

use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use std::num::IntErrorKind;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Duration;
use tokio::time::sleep;

use routes::lamp_routes;
use routes::config_routes;
use routes::weather_routes;

use devices::lamps::Lamps;
use devices::shellies::Shellies;
use information::weather::FrontendWeather;
 

struct ActixAppState {
    pub shellies: Mutex<Shellies>,
    pub lamps: Mutex<Lamps>,
    pub weather: Arc<Mutex<FrontendWeather>>
}

async fn weather_loop(weather: Arc<Mutex<FrontendWeather>>) {
    let interval = std::env::var("WEATHER_UPDATE_INTERVAL")
        .expect("Error in .env: WEATHER_UPDATE_INTERVAL not found")
        .parse::<u64>()
        .expect("Error in .env ");
    
    loop {
        let _ = weather.lock().await.update().await;
        sleep(Duration::from_secs(interval)).await;
    }

}

pub fn run_webserver(
    listener: TcpListener,
    shellies: Shellies,
    lamps: Lamps,
    frontend_weather: FrontendWeather,
    ) -> Result<Server, std::io::Error> {

    let frontend_weather_mutex = Arc::new(Mutex::new(frontend_weather));
    
    // only start the openweather retrievement loop unless not disabled via environment variable
    match std::env::var("DISABLE_WEATHER_RETRIEVEMENT") {
        Ok(disable_weather_retrievement) if disable_weather_retrievement.to_lowercase() == "true" => {
            println!("Found DISABLE_WEATHER_RETRIEVEMENT=true. Not retrieving information from api.openweather.org");
        }
        _ => {
            tokio::spawn(weather_loop(frontend_weather_mutex.clone()));
        } 
    }

    let actix_app_state = web::Data::new(ActixAppState {
        shellies: Mutex::new(shellies),
        lamps: Mutex::new(lamps),
        weather: frontend_weather_mutex,
    });

    let server = HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(actix_app_state.clone())
            .service(config_routes::health_check)
            
            .service(config_routes::get_config)
            .service(config_routes::write_config)
            
            .service(config_routes::add_shelly)
            .service(config_routes::delete_shelly)
            .service(config_routes::modify_shelly)

            .service(config_routes::add_lamp)
            .service(config_routes::delete_lamp)
            .service(config_routes::modify_lamp)

            .service(lamp_routes::lamps_all_off)
            .service(lamp_routes::lamp_toggle)
            .service(lamp_routes::lamp_on)
            .service(lamp_routes::lamp_set_brightness)
            .service(lamp_routes::get_lamps)
            .service(lamp_routes::get_lamps_force_backend_sync)
            
            .service(weather_routes::get_weather)

    })
    .listen(listener)?
    .run();

    Ok(server)
}
