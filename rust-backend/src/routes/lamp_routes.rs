use actix_web::{get, web, HttpResponse, Responder};
use crate::devices::frontend_structs::*;
use crate::devices::shellies::State;
use crate::ActixAppState;


// switched
#[get("/lamp_toggle/{id}")]
async fn lamp_toggle(shelly_app_state: web::Data<ActixAppState>, path: web::Path<usize>) -> impl Responder {
    println!("Inside toggle lamp. path: {:?}", &path);
    let mut lamps = shelly_app_state.lamps.lock().await;
    let mut shellies = shelly_app_state.shellies.lock().await;
    
    let id = path.into_inner();
    let lamp = match lamps.get_mut_lamp_by_id(id) {
        Err(e) => {
            eprintln!("Error: Lamp with id {} not found:\n{}", id, e);
            return HttpResponse::BadRequest().body("Bad Request");
        },
        Ok(lamp) => lamp,

    };

    match lamp.switch(&State::Toggle, &mut shellies).await {
        Err(e) => {
            eprintln!("Error in toggle_lamp with lamp id \"{}\": {}", &lamp.id, e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        },
        Ok((state, brightness)) =>  {
            let state = matches!(state, State::On);
            let frontend_lamp = FrontendLamp { id: lamp.id, name: lamp.name.clone(), is_on: state, brightness };
            HttpResponse::Ok().json(frontend_lamp)
        }

    }
}

// switched to id
#[get("/lamp_on/{id}")]
async fn lamp_on(shelly_app_state: web::Data<ActixAppState>, path: web::Path<usize>) -> impl Responder {
    println!("Inside toggle lamp. path: {:?}", &path);
    let mut lamps = shelly_app_state.lamps.lock().await;
    let mut shellies = shelly_app_state.shellies.lock().await;
    
    let id = path.into_inner();
    let lamp = match lamps.get_mut_lamp_by_id(id) {
        Err(e) => {
            eprintln!("Error: Lamp with id {} not found:\n{}",id, e);
            return HttpResponse::BadRequest().body("Bad Request");
        },
        Ok(lamp) => lamp,
    };

    match lamp.switch(&State::On, &mut shellies).await {
        Err(e) => {
            eprintln!("Error in toggle_lamp with lamp id \"{}\": {}", &lamp.id, e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        },
        Ok((state, brightness)) =>  {
            let state = matches!(state, State::On);
            let frontend_lamp = FrontendLamp { id: lamp.id, name: lamp.name.clone(), is_on: state, brightness };
            HttpResponse::Ok().json(frontend_lamp)
        }
    }
}

#[get("/lamps_all_off")]
async fn lamps_all_off(shelly_app_state: web::Data<ActixAppState>) -> impl Responder {
    println!("Inside Lamps all off lamp. path");
    let mut lamps = shelly_app_state.lamps.lock().await;
    let mut shellies = shelly_app_state.shellies.lock().await;
    
    match lamps.global_off(&mut shellies).await {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        Ok(_) => {
             let frontend_lamps = FrontendLamps::get_frontend_lamps(&lamps, &shellies).frontend_lamps;
     
            HttpResponse::Ok().json(frontend_lamps)
        }
    }
    
}

// switched to id
#[get("/lamp_set_brightness/{id}/{brightness}")]
async fn lamp_set_brightness(shelly_app_state: web::Data<ActixAppState>, path: web::Path<(usize, usize)>) -> impl Responder {
    println!("Inside lamp_set_brightness(). path: {:?}", &path);

    let (id, brightness) = path.into_inner();
    let lamps = shelly_app_state.lamps.lock().await;
    let mut shellies = shelly_app_state.shellies.lock().await;

    let lamp = match lamps.get_lamp_by_id(id) {
        Ok(lamp) => lamp,
        Err(e) => {
            eprintln!("Error: Lamp not found:\n{}", e);
            return HttpResponse::BadRequest();
        }
    };

    if brightness > 100 {
        eprintln!("Error: Brightness out of bounds:\n{}", brightness);
        return HttpResponse::BadRequest();
    }
    
    match lamp.set_brightness(brightness, &mut shellies).await {
        Err(e) => {
            eprintln!("Error: Couldn't set brightness.\n{:?}", e);
            HttpResponse::InternalServerError()
        }
        Ok(_) => HttpResponse::Ok()
    }
}



#[get("/get_lamps")]
pub async fn get_lamps(shelly_app_state: web::Data<ActixAppState>) -> actix_web::Result<impl Responder> {
    println!("Inside get_lamps");
    let lamps = shelly_app_state.lamps.lock().await;
    let shellies = shelly_app_state.shellies.lock().await;
    
    let frontend_lamps = FrontendLamps::get_frontend_lamps(&lamps, &shellies).frontend_lamps;
    Ok(web::Json(frontend_lamps))
}

#[get("/get_lamps_force_backend_sync")]
pub async fn get_lamps_force_backend_sync(shelly_app_state: web::Data<ActixAppState>) -> actix_web::Result<impl Responder> {
    println!("Inside force_backend_sync");
    let lamps = shelly_app_state.lamps.lock().await;
    let mut shellies = shelly_app_state.shellies.lock().await;
    let _ = shellies.get_states().await;
    let frontend_lamps = FrontendLamps::get_frontend_lamps(&lamps, &shellies).frontend_lamps;
    Ok(web::Json(frontend_lamps))
}


