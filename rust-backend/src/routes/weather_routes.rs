use actix_web::{get, web, Responder};
use crate::ActixAppState;

#[get("/get_weather")]
pub async fn get_weather(shelly_app_state: web::Data<ActixAppState>) -> actix_web::Result<impl Responder> {
    let weather = shelly_app_state.weather.lock().await;
    Ok(web::Json(weather.clone()))
}