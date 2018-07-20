extern crate temperaturecollector;
extern crate config;
extern crate glob;
extern crate clap;


use temperaturecollector::Config;
use temperaturecollector::DatagramListener;
use std::net::IpAddr;
use glob::glob;
use config::*;
use clap::{Arg, App};
use std::str::FromStr;
use std::error::Error;

fn apply_command_line_arguments(mut config: Config) -> Result<Config, Box<Error>> {
    let matches = App::new("Temperature data collector")
        .about("This is a service that receives datas from the sensors and sents to redis")
        .arg(Arg::with_name("server_ip")
            .short("h")
            .long("host")
            .value_name("server_ip")
            .help("The IP address of the service")
            .takes_value(true))
        .arg(Arg::with_name("server_port")
            .short("p")
            .long("port")
            .value_name("server_port")
            .help("The (UDP) port of the service")
            .takes_value(true))
        .arg(Arg::with_name("buffer_size")
            .long("buffer-size")
            .value_name("buffer_size")
            .help("The size of the UDP buffer")
            .takes_value(true))
        .arg(Arg::with_name("log_datagrams")
            .long("log-datagrams")
            .value_name("log_datagrams")
            .help("True if to log the received datagrams")
            .takes_value(true))
        .arg(Arg::with_name("log_redis_query")
            .long("log-redis-query")
            .value_name("log_redis_query")
            .help("True if to log used redis query")
            .takes_value(true))
        .arg(Arg::with_name("redis_url")
            .long("redis-url")
            .value_name("redis_url")
            .help("The URL used for connecting to redis")
            .takes_value(true))
        .arg(Arg::with_name("max_time_to_live")
            .long("max-time-to-live")
            .value_name("max_time_to_live")
            .help("Maximium life of the data")
            .takes_value(true))
        .get_matches();

    match matches.value_of("server_ip") {
        Some(val) => { config.server_ip = IpAddr::from_str(val)? },
        None => { }
    }
    match matches.value_of("server_port") {
        Some(val) => { config.server_port = u16::from_str(val)? },
        None => { }
    }
    match matches.value_of("buffer_size") {
        Some(val) => { config.buffer_size = u16::from_str(val)? },
        None => { }
    }
    match matches.value_of("log_datagrams") {
        Some(val) => { config.log_datagrams = bool::from_str(val)? },
        None => { }
    }
    match matches.value_of("log_redis_query") {
        Some(val) => { config.log_redis_query = bool::from_str(val)? },
        None => { }
    }
    match matches.value_of("redis_url") {
        Some(val) => { config.redis_url = val.to_string() },
        None => { }
    }
    match matches.value_of("max_time_to_live") {
        Some(val) => { config.max_time_to_live = u32::from_str(val)? },
        None => { }
    }
    Ok(config)
}

fn load_settings() -> Result<Config, ConfigError> {
    let mut settings = config::Config::default();

    settings
        .set_default("buffer_size", "2048")?
        .set_default("log_datagrams", "false")?
        .set_default("log_redis_query", "false")?
        .set_default("redis_url", "redis://127.0.0.1")?
        .set_default("server_ip", "0.0.0.0")?
        .set_default("server_port", "8886")?
        .set_default("max_time_to_live", "60")?
        .merge(config::File::with_name("default").required(false))?
        .merge(config::File::with_name("conf").required(false))?
        .merge(glob("conf.d/*")
                   .unwrap()
                   .map(|path| File::from(path.unwrap()))
                   .collect::<Vec<_>>())?
        .merge(config::Environment::with_prefix("TEMPERATURECOLLECTOR"))?;

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
    let cfg = apply_command_line_arguments(cfg).unwrap();
    //println!("{:?}", cfg);
    DatagramListener::run(&cfg);
}
