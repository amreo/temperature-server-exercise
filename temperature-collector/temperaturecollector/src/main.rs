extern crate temperaturecollector;
extern crate config;
extern crate glob;

use temperaturecollector::Config;
use temperaturecollector::DatagramListener;
use std::net::IpAddr;
use glob::glob;
use config::*;

fn load_settings() -> Result<Config, ConfigError> {
    let mut settings = config::Config::default();

    settings
        .set_default("buffer_size", "2048")?
        .set_default("log_datagrams", "true")?
        .set_default("log_redis_query", "true")?
        .set_default("redis_url", "redis://127.0.0.1")?
        .set_default("server_ip", "0.0.0.0")?
        .set_default("server_port", "8886")?
        .set_default("max_time_to_live", "60")?
        .merge(config::File::with_name("default"))?
        .merge(config::File::with_name("conf"))?
        .merge(glob("conf.d/*")
                   .unwrap()
                   .map(|path| File::from(path.unwrap()))
                   .collect::<Vec<_>>())?
        .merge(config::Environment::with_prefix("TEMPERATURECOLLECTOR"))?;

    
    // Print out our settings (as a HashMap)
    //println!("{:?}",
    //         settings.try_into::<HashMap<String, String>>().unwrap());

    

    Ok(Config {
        buffer_size: settings.get::<u16>("buffer_size")?,
        log_datagrams: settings.get_bool("log_datagrams")?,
        log_redis_query: settings.get_bool("log_redis_query")?,
        redis_url: settings.get_str("redis_url")?,
        server_ip: settings.get::<IpAddr>("server_ip")?,
        server_port: settings.get::<u16>("server_port")?,
        max_time_to_live: settings.get::<u32>("max_time_to_live")?,
    })
}

fn main() {
    let cfg = load_settings().unwrap();
    println!("{:?}", cfg);
    DatagramListener::run(&cfg);
}
