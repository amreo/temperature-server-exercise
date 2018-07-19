use rocket::error::LaunchError;
use rocket::fairing::AdHoc;
use rocket::ignite;
use rocket_contrib::Json;
mod redisdbconn;
pub use self::redisdbconn::{DbConn, init_pool};
use redis::cmd;
use std::ops::Deref;
use ::TemperatureData;
use std::collections::HashMap;
use std::str::FromStr;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};
use rocket::http::Method;

fn temperature_list(conn: DbConn) -> Vec<TemperatureData> {
    let mut list =  cmd("KEYS").arg("temperaturedata:*").query::<HashMap<String, String>>(conn.deref()).expect("Error in getting the keys list").keys().filter_map(|key| {
        let res = cmd("HGETALL").arg(key).query::<HashMap<String, String>>(conn.deref());
        
        match res {
            Ok(val) => {
                if val.len() == 8 {
                    Some(TemperatureData::new(
                        u16::from_str(val.get("sensorID").expect("SensorID should be present")).expect("Parsing error in sensorID"), 
                        i32::from_str(val.get("year").expect("Year should be present")).expect("Parsing error in year"), 
                        u8::from_str(val.get("month").expect("Month should be present")).expect("Parsing error in month"), 
                        u8::from_str(val.get("day").expect("Day should be present")).expect("Parsing error in day"), 
                        u8::from_str(val.get("hour").expect("Hour should be present")).expect("Parsing error in hour"), 
                        u8::from_str(val.get("minute").expect("Minute should be present")).expect("Parsing error in minute"), 
                        u8::from_str(val.get("second").expect("Second should be present")).expect("Parsing error in second"), 
                        f32::from_str(val.get("temperature").expect("Temperature should be present")).expect("Parsing error in temperature"))
                    )
                } else {
                    None
                }
            }, Err(_) => None
        }
        
    }).collect::<Vec<TemperatureData>>();
    list.sort_unstable();
    list
} 


#[get("/temperatures")]
fn get_temperature_list(conn: DbConn) -> Json<Vec<TemperatureData>> {
    Json(temperature_list(conn))
}
#[get("/temperatures/<id>")]
fn get_temperate_list_by_sensor_id(conn: DbConn, id: u16) -> Json<Vec<TemperatureData>> {
    Json(temperature_list(conn).into_iter().filter(|item| item.sensor_id() == id).collect())
}

pub fn launch() -> LaunchError {
    let allowed_origins= AllowedOrigins::all();
    // You can also deserialize this
    let options = Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    };

    ignite()
        .attach(AdHoc::on_attach(|rocket| {
            let redis_url = rocket.config().get_str("redis_url").unwrap_or("redis://127.0.0.1").to_string();
            Ok(rocket.manage(init_pool(redis_url.as_str())))
        }))
        .attach(options)
        .mount("/api", routes![get_temperature_list, get_temperate_list_by_sensor_id]).launch()
}