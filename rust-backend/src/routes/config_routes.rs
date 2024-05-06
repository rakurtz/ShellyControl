use actix_web::{get, post, delete, patch, web, HttpResponse, Responder};
use crate::config::{ConfigShelly, FromConfig};
use crate::devices::frontend_structs::*;
use crate::devices::lamps::Lamp;

use crate::ActixAppState;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/config")]
pub async fn get_config(shelly_app_state: web::Data<ActixAppState>) -> actix_web::Result<impl Responder> {
    let lamps = shelly_app_state.lamps.lock().await;
    let shellies = shelly_app_state.shellies.lock().await;   
    Ok(web::Json(FrontendConfig{shellies: shellies.shellies.clone(), lamps: lamps.lamps.clone()}))
}

#[post("/config")]
pub async fn write_config(received_config: web::Json<FrontendConfig>, shelly_app_state: web::Data<ActixAppState>) -> impl Responder {
    println!("Inside write_config");
    // dbg!(&received_config);

    let mut lamps = shelly_app_state.lamps.lock().await;
    let mut shellies = shelly_app_state.shellies.lock().await;
    let received_config = received_config.into_inner();
    lamps.lamps = received_config.lamps.clone();
    shellies.shellies = received_config.shellies;

    // write to yaml
    let config = FromConfig::new_from_state(shellies.shellies.clone(), lamps.lamps.clone());
    let _ = config.write_to_yaml();
    HttpResponse::Ok()
}

//
// Shellies
//
#[post("/shelly")]
async fn add_shelly(received_shelly: web::Json<ConfigShelly>, shelly_app_state: web::Data<ActixAppState>) -> impl Responder {
    println!("Received {:?}!", received_shelly);
    let mut shellies = shelly_app_state.shellies.lock().await;
   
    let new_id = shellies.add_from_config_shelly(received_shelly.into_inner());

    println!("added shelly to internal state with id: {}", new_id);
    HttpResponse::Ok()

}

#[delete("/shelly/{id}")]
async fn delete_shelly(shelly_app_state: web::Data<ActixAppState>, path: web::Path<usize>) -> impl Responder {
    let id = path.into_inner();
    let mut shellies = shelly_app_state.shellies.lock().await;
    if shellies.delete_shelly_by_id(id).is_err() {
        eprintln!("Couldn't find Shelly by id {}", id);
        return HttpResponse::NotFound();    
    }
    println!("Shelly with id {} deleted.", id);
    HttpResponse::Ok()
}

#[patch("/shelly")]
async fn modify_shelly(received_shelly: web::Json<ConfigShelly>, shelly_app_state: web::Data<ActixAppState>) -> impl Responder {
    let mut shellies = shelly_app_state.shellies.lock().await;
    if shellies.modify_shelly_by_id(received_shelly.into_inner()).is_err() {
        eprintln!("Couldn't change Shelly with given id.");
        return HttpResponse::NotFound();    
    }
    println!("Shelly with given id succesfully updated.");
    HttpResponse::Ok()
}

//
// Lamps
// 
#[post("/lamp")]
async fn add_lamp(received_lamp: web::Json<Lamp>, shelly_app_state: web::Data<ActixAppState>) -> impl Responder {
    println!("Received {:?}!", received_lamp);
    let mut lamps = shelly_app_state.lamps.lock().await;
   
    let new_id = lamps.add_lamp(received_lamp.into_inner());

    println!("added lamp to internal state with id: {}", new_id);
    HttpResponse::Ok()
}

#[delete("/lamp/{id}")]
async fn delete_lamp(shelly_app_state: web::Data<ActixAppState>, path: web::Path<usize>) -> impl Responder {
    let id = path.into_inner();
    let mut lamps = shelly_app_state.lamps.lock().await;
    if lamps.delete_lamp_by_id(id).is_err() {
        eprintln!("Couldn't find Lamp by id {}", id);
        return HttpResponse::NotFound();    
    }
    println!("Lamp with id {} deleted.", id);
    HttpResponse::Ok()
}

#[patch("/lamp")]
async fn modify_lamp(received_lamp: web::Json<Lamp>, shelly_app_state: web::Data<ActixAppState>) -> impl Responder {
    let mut lamps = shelly_app_state.lamps.lock().await;
    if lamps.modify_lamp_by_id(received_lamp.into_inner()).is_err() {
        eprintln!("Couldn't change Lamp with given id.");
        return HttpResponse::NotFound();    
    }
    println!("Lamp with given id succesfully updated.");
    HttpResponse::Ok()
}