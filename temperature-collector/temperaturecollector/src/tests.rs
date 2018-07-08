#[cfg(test)]
mod tests {
    use ::TemperatureData;
    use chrono::prelude::*;

    #[test]
    fn temperature_data_new_should_return_a_new_temperature_data() {
        let dt = Local::now();
        let temperature_data = TemperatureData::new(10, dt, 22.5);

        assert_eq!(temperature_data.sensor_id, 10);
        assert_eq!(temperature_data.temperature, 22.5);
        assert_eq!(temperature_data.time, dt);
    }
}