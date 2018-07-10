use rocket::error::LaunchError;
use rocket::fairing::AdHoc;
use rocket::ignite;
use rocket_contrib::Json;
mod redisdbconn;
pub use self::redisdbconn::{DbConn, init_pool};
use redis::cmd;
use std::ops::Deref;
use ::TemperatureData;
use std::collections::{LinkedList, HashMap};
use std::str::FromStr;
#[get("/")]
fn temperature_list(conn: DbConn) -> Json<LinkedList<TemperatureData>> {
    Json(cmd("KEYS").arg("temperaturedata:*").query::<HashMap<String, String>>(conn.deref()).unwrap().keys().filter_map(|key| {
        let res = cmd("HGETALL").arg(key).query::<HashMap<String, String>>(conn.deref());
        match res {
            Ok(val) => Some(TemperatureData::new(
                u16::from_str(val.get("sensorID").unwrap()).unwrap(), 
                i32::from_str(val.get("year").unwrap()).unwrap(), 
                u8::from_str(val.get("month").unwrap()).unwrap(), 
                u8::from_str(val.get("day").unwrap()).unwrap(), 
                u8::from_str(val.get("hour").unwrap()).unwrap(), 
                u8::from_str(val.get("minute").unwrap()).unwrap(), 
                u8::from_str(val.get("second").unwrap()).unwrap(), 
                f32::from_str(val.get("temperature").unwrap()).unwrap())
            ), Err(_) => None
        }
        
    }).collect())
}

pub fn launch() -> LaunchError {
    ignite()
        .attach(AdHoc::on_attach(|rocket| {
            let redis_url = rocket.config().get_str("redis_url").unwrap_or("redis://127.0.0.1").to_string();
            Ok(rocket.manage(init_pool(redis_url.as_str())))
        }))
        .mount("/temperature_list", routes![temperature_list]).launch()
}