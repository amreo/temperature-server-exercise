#[derive(Debug, Serialize)]
pub struct TemperatureData {
    sensor_id: u16,
    time_year: i32,
    time_month: u8,
    time_day: u8,
    time_hour: u8,
    time_minute: u8,
    time_second: u8,
    temperature: f32,
}

impl TemperatureData {
    //Create a new temperature data
    pub fn new(sensor_id: u16, time_year: i32, time_month: u8, time_day: u8, time_hour: u8, time_minute: u8, time_second: u8, temperature: f32) -> TemperatureData{
        TemperatureData { 
            sensor_id: sensor_id, 
            time_year: time_year,
            time_month: time_month,
            time_day: time_day,
            time_hour: time_hour,
            time_minute: time_minute,
            time_second: time_second,
            temperature: temperature
        }
    }

    pub fn sensor_id(&self) -> u16 { self.sensor_id } 
    pub fn temperature(&self) -> f32 { self.temperature } 
    pub fn time_year(&self) -> i32 { self.time_year } 
    pub fn time_month(&self) -> u8 { self.time_month }
    pub fn time_day(&self) -> u8 { self.time_day }
    pub fn time_hour(&self) -> u8 { self.time_hour }
    pub fn time_minute(&self) -> u8 { self.time_minute }
    pub fn time_second(&self) -> u8 { self.time_second }
}