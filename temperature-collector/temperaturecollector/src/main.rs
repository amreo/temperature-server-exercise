extern crate temperaturecollector;
use temperaturecollector::Config;
use temperaturecollector::DatagramListener;
use std::net::IpAddr;
fn main() {
    let cfg = Config {
        buffer_size: 2048,
        log_datagrams: true,
        log_redis_query: true,
        redis_url: String::from("redis://127.0.0.1"),
        server_ip: IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
        server_port: 8886
    };

    DatagramListener::run(&cfg);
}
