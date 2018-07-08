mod tests;

extern crate chrono;
extern crate regex;

use chrono::prelude::*;
use regex::Regex;
use std::str::FromStr;
//Data of any temperature
pub struct TemperatureData {
    pub sensor_id: u16,
    pub time: NaiveDateTime,
    pub temperature: f32,
}

impl TemperatureData {
    //Create a new temperature data
    pub fn new(sensor_id: u16, time: NaiveDateTime, temperature: f32) -> TemperatureData{
        TemperatureData { 
            sensor_id: sensor_id, 
            time: time,
            temperature: temperature
        }
    }

    pub fn from_datagram(datagram: &str) -> Result<TemperatureData, String> {
        let regex_sensore = Regex::new(r"^sensore (\d+);$").unwrap();
        let regex_data = Regex::new(r"^data (\d+)-(\d+)-(\d+);$").unwrap();
        let regex_ora = Regex::new(r"^ora (\d+)-(\d+)-(\d+);$").unwrap();
        let regex_temperatura_rilevata = Regex::new(r"^temperatura rilevata ([+-]?([0-9]*[.])?[0-9]+);$").unwrap();
        let mut sensor_id = 0;
        let mut date_time_year = 2015;
        let mut date_time_month = 3;
        let mut date_time_day = 15;
        let mut date_time_hour = 1;
        let mut date_time_minute = 1;
        let mut date_time_second = 1; 
        let mut temperature = 0.0;

        for line in datagram.lines() {
            if line == "inizio;" {
                //Ignored
            } else if line == "fine" {
                //Ignored
            } else if regex_sensore.is_match(line) {
                sensor_id = u16::from_str(regex_sensore.captures_iter(line).next().unwrap().get(1).unwrap().as_str()).unwrap();
            } else if regex_temperatura_rilevata.is_match(line) {
                temperature = f32::from_str(regex_temperatura_rilevata.captures_iter(line).next().unwrap().get(1).unwrap().as_str()).unwrap();
            } else if regex_data.is_match(line) {
                let group = regex_data.captures_iter(line).next().unwrap();
                date_time_year = i32::from_str(group.get(3).unwrap().as_str()).unwrap(); 
                date_time_month = u32::from_str(group.get(2).unwrap().as_str()).unwrap(); 
                date_time_day = u32::from_str(group.get(1).unwrap().as_str()).unwrap(); 
            } else if regex_ora.is_match(line) {
                let group = regex_ora.captures_iter(line).next().unwrap();
                date_time_hour = u32::from_str(group.get(1).unwrap().as_str()).unwrap(); 
                date_time_minute = u32::from_str(group.get(2).unwrap().as_str()).unwrap(); 
                date_time_second = u32::from_str(group.get(3).unwrap().as_str()).unwrap(); 
            } else {
                return Err(format!("error in parsing a datagram's line: {}", line));
            }
        }

        Ok(TemperatureData { 
            sensor_id: sensor_id,
            time: NaiveDate::from_ymd(date_time_year, date_time_month, date_time_day).and_hms(date_time_hour, date_time_minute, date_time_second),
            temperature: temperature
        })
    }
}

