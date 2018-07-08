#[cfg(test)]
mod tests {
    use ::TemperatureData;
    use chrono::prelude::*;

    #[test]
    fn temperature_data_new_should_return_a_new_temperature_data() {
        let dt = NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11);
        let temperature_data = TemperatureData::new(10, dt, 22.5);

        assert_eq!(temperature_data.sensor_id, 10);
        assert_eq!(temperature_data.temperature, 22.5);
        assert_eq!(temperature_data.time, dt);
    }

    #[test]
    fn temperature_data_from_datagram_should_return_a_new_temperature_data() {
        let datagram = "inizio;
sensore 10;
data 8-7-2016;
ora 9-10-11;
temperatura rilevata 22.5;
fine";
        
        let temperature_data = TemperatureData::from_datagram(datagram);
        assert!(temperature_data.is_ok());
        let temperature_data = temperature_data.unwrap();
        assert_eq!(temperature_data.sensor_id, 10);
        assert_eq!(temperature_data.temperature, 22.5);
        assert_eq!(temperature_data.time, NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11));
    }

    #[test]
    fn temperature_data_from_datagram_should_return_a_error() {
        let datagram = "inizio;
sensosfdre 10;
data 8-7-2016;
ora 9-10-11;
temperatura rilevata 22.5;
fine";
        
        let temperature_data = TemperatureData::from_datagram(datagram);
        assert!(temperature_data.is_err());
    }
}