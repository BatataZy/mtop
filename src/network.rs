use std::{net, process, str::FromStr};

use curl::easy::Easy;

use crate::unit_types::Magnitude;

#[derive(Debug)]
pub struct Adapter {
    pub name: String,
    pub interface: String,
    pub local_ip: net::Ipv4Addr,
    pub public_ip: net::Ipv4Addr,
}

impl Adapter {
    fn new() -> Adapter {
        Adapter {
            name: String::new(),
            interface: String::new(),
            local_ip: net::Ipv4Addr::new(0, 0, 0, 0),
            public_ip: net::Ipv4Addr::new(0, 0, 0, 0)
        }
    }
}


pub struct Network {
    up: Magnitude,
    down: Magnitude,
    pub adapter: Adapter
}

impl Network {
    pub fn new() -> Network {
        Network {
            up: Magnitude::new(),
            down: Magnitude::new(),
            adapter: Adapter::new(),
        }
    }

    pub async fn ip_update(&mut self) {

        let adapters = process::Command::new("nmcli").args(["-t", "-f", "device", "device"]).output().unwrap().stdout;

        let current_adapter_name = std::str::from_utf8(&adapters).unwrap().split('\n').collect::<Vec<&str>>()[0].to_owned();

        let current_local_ip = &String::from_utf8(
            process::Command::new("nmcli").args(["-g", "IP4.ADDRESS", "device", "show", &current_adapter_name])
            .output().unwrap().stdout.split(|x| x == &47).nth(0).unwrap().to_owned()
            ).unwrap();

        if self.adapter.name != current_adapter_name && current_local_ip != "\n" {

            self.adapter.name = current_adapter_name;
            
            self.adapter.interface = String::from_utf8(
                                    process::Command::new("nmcli").args(["-g", "GENERAL.TYPE", "device", "show", &self.adapter.name])
                                        .output().unwrap().stdout.split(|x| x == &10).nth(0).unwrap().to_owned()
                                    ).unwrap();

            self.adapter.local_ip = net::Ipv4Addr::from_str(&current_local_ip).unwrap_or(net::Ipv4Addr::new(0, 0, 0, 0));

            self.adapter.public_ip = net::Ipv4Addr::from_str(
                                        &curl("https://api.ipify.org")
                                    ).unwrap_or(net::Ipv4Addr::new(0, 0, 0, 0));
        }
    }
}

fn curl(url: &str) -> String {

    let mut easy = Easy::new();
    let mut buffer: Vec<u8> = Vec::new();

    easy.url(url).unwrap();

    {
        let mut transfer = easy.transfer();

        transfer.write_function(|data| {
            buffer.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();

        transfer.perform().unwrap_or_default();
    }

    return String::from_utf8(buffer).unwrap();
}