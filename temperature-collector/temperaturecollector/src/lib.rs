mod tests;

extern crate chrono;
use chrono::prelude::*;

//Data of any temperature
pub struct TemperatureData {
    sensor_id: u16,
    time: DateTime<Local>,
    temperature: f32,
}

impl TemperatureData {
    pub fn new(sensor_id: u16, time: DateTime<Local>, temperature: f32) -> TemperatureData{
        TemperatureData { 
            sensor_id: sensor_id, 
            time: time,
            temperature: temperature
        }
    }
}

