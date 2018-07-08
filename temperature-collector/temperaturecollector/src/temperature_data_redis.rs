use ::{TemperatureData, Config};
use redis::{Client, RedisResult, pipe};
use chrono::prelude::*;
use rand::{thread_rng, Rng};

pub fn temperature_data_save(td: &TemperatureData, config: &Config) -> RedisResult<()> {
    let client = try!(Client::open(config.redis_url.as_str()));
    let con  = try!(client.get_connection());
    let key = format!("temperaturedata:{:04}{:02}{:02}{:02}{:02}{:02}{:06}", td.time().year(), td.time().month(), td.time().day(), td.time().hour(), td.time().minute(), td.time().second(), thread_rng().gen_range(1, 1000000));

    //Commands: 
    //HSET temperaturedata:[score] 
    //  temperature [temperature] sensorID [sensorID] year [year] month [month] day [day] hour [hour] minute [minute] second [second]
    //EXPIRE temperaturedata:[score] [config.max_time_to_live]
    if config.log_redis_query {
        println!("HSET {} temperature {} sensorID {} year {} month {} day {} hour {} minute {} second {}
EXPIRE {} {}", key, td.temperature(), td.sensor_id(), td.time().year(), td.time().month(), td.time().day(),
            td.time().hour(), td.time().minute(), td.time().second(), key, config.max_time_to_live);
    }

    let _ : () = try!(pipe()
        .cmd("HSET").arg(&key).arg("temperature").arg(td.temperature())
            .arg("sensorID").arg(td.sensor_id())
            .arg("year").arg(td.time().year())
            .arg("month").arg(td.time().month())
            .arg("day").arg(td.time().day())
            .arg("hour").arg(td.time().hour())
            .arg("minute").arg(td.time().minute())
            .arg("second").arg(td.time().second())
            .ignore()
        .cmd("EXPIRE").arg(&key).arg(config.max_time_to_live).ignore()
        .query(&con));
    
    Ok(())
}