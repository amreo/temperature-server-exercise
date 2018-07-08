use ::Config;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::str;
use ::TemperatureData;
pub struct DatagramListener {
    
}

impl DatagramListener {
    pub fn run(config: &Config) {
        let endpoint = SocketAddr::new(config.server_ip, config.server_port);
        let socket = match UdpSocket::bind(endpoint) {
            Ok(s) => s,
            Err(e) => { 
                panic!("couldn't bind socket: {}", e);
            }
        };

        let mut buf = [0; 2048];
        loop {
            match socket.recv_from(&mut buf) {
                Ok((length, src)) => {
                    if config.log_datagrams { println!("received datagram from: {}", src); }
                    let td = match TemperatureData::from_datagram(&str::from_utf8(&buf[..]).unwrap()[..length]) {
                        Ok(td) => {
                            //Print the data
                            if config.log_datagrams { println!("{:?}", td); }
                            //Save the data
                        },
                        Err(e) => {
                            eprintln!("Error in receiving of the datagram: {}", e);
                        }
                    };
                },
                Err(e) => {
                    eprintln!("couldn't recieve a datagram: {}", e);
                }
            }
        }

    }


}