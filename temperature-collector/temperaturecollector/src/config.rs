use std::net::IpAddr;

pub struct Config {
    pub redis_url: String,
    pub server_ip: IpAddr,
    pub server_port: u16,
    pub log_datagrams: bool,
    pub log_redis_query: bool,
    pub buffer_size: u16
}