use std::net::TcpListener;
// use dotenvy::dotenv;

use shelly_actix_api::run_webserver;
use shelly_actix_api::config::FromConfig;
use shelly_actix_api::devices::lamps::Lamps;
use shelly_actix_api::devices::shellies::Shellies;
use shelly_actix_api::information::weather::FrontendWeather;



#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = FromConfig::read_config_shellies().expect("ERROR: Couldn't parse config file. Also check configuration.env and compose.yaml");
    let shellies = Shellies::new_from_config_shellies(config.shellies).await;
    let lamps = Lamps {lamps: config.lamps};
    let weather = FrontendWeather::new();
    
    let listener = TcpListener::bind("0.0.0.0:3000").expect("TCPListener: failed to bind to address.");
    run_webserver(listener, shellies, lamps, weather)?.await
}
 