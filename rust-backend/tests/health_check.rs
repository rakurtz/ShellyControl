// testing health_check() via external request on the actual server
// here, tokio::spawn() is used to spin up our app in the background

use std::net::TcpListener;
use shelly_actix_api::config::FromConfig;
use shelly_actix_api::devices::lamps::Lamps;
use shelly_actix_api::devices::shellies::Shellies;
use shelly_actix_api::information::weather::FrontendWeather;


#[tokio::test]
async fn health_check_works() {
    // arrange
    let local_address = spawn_app().await;
    dbg!(&local_address);
    let client = reqwest::Client::new();
    
    // act
    let response = client.get(&format!("{}/health_check", &local_address)) 
        .send()
        .await
        .expect("Failed to execute request.");

    dbg!(&response);
    // assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

async fn spawn_app() -> String {
    // run server at port "0" to let the OS find an available port for us
    // this is to prevent failing tests due to ports already beeing occupied
    // when running multiple tests at the same time...
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    // load .env and read config
    dotenvy::dotenv().ok();
    let config = FromConfig::read_config_shellies().expect("Couldn't parse config file.");
    
    let shellies = Shellies::new_from_config_shellies(config.shellies).await;
    let lamps = Lamps {lamps: config.lamps};
    let weather = FrontendWeather::new();

    let server = shelly_actix_api::run_webserver(listener, shellies, lamps, weather).expect("could spawn Server!");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}