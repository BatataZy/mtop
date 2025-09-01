use std::{net, path::PathBuf, str::FromStr};

use curl::easy::Easy;

use crate::{run, unit_types::Magnitude};

#[derive(Debug)]
pub struct Adapter {
    pub name: String,
    pub interface: String,
    pub local_ip: net::Ipv4Addr,
    pub public_ip: net::Ipv4Addr,
}

impl Adapter {
    const fn new() -> Self {
        Self {
            name: String::new(),
            interface: String::new(),
            local_ip: net::Ipv4Addr::UNSPECIFIED,
            public_ip: net::Ipv4Addr::UNSPECIFIED,
        }
    }
}

pub struct Network {
    _up: Magnitude,
    _down: Magnitude,
    pub adapter: Adapter,
}

impl Network {
    pub fn new() -> Self {
        Self {
            _up: Magnitude::new(&PathBuf::new()),
            _down: Magnitude::new(&PathBuf::new()),
            adapter: Adapter::new(),
        }
    }

    pub fn ip_update(&mut self) {
        let adapters = run("nmcli -t -f device device").stdout;

        let current_adapter_name: String = String::from_utf8_lossy(&adapters)
            .split('\n')
            .collect::<Vec<&str>>()[0]
            .to_owned();

        let current_local_ip: net::Ipv4Addr = net::Ipv4Addr::from_str(
            String::from_utf8_lossy(
                &run(&("nmcli -g IP4.ADDRESS device show ".to_owned() + &current_adapter_name))
                    .stdout,
            )
            .trim_suffix("/24\n"),
        )
        .unwrap_or(net::Ipv4Addr::UNSPECIFIED);

        if self.adapter.name != current_adapter_name || self.adapter.local_ip != current_local_ip {
            self.adapter.name = current_adapter_name;

            self.adapter.interface = String::from_utf8_lossy(
                &run(&("nmcli -g GENERAL.TYPE device show ".to_owned() + &self.adapter.name))
                    .stdout,
            )
            .trim_end()
            .to_string();

            self.adapter.local_ip = current_local_ip;

            self.adapter.public_ip = net::Ipv4Addr::from_str(&curl("https://api.ipify.org"))
                .unwrap_or(net::Ipv4Addr::UNSPECIFIED);
        }
    }
}

fn curl(url: &str) -> String {
    let mut easy = Easy::new();
    let mut buffer: Vec<u8> = Vec::new();

    easy.url(url).expect("Couldn't get URL");

    {
        let mut transfer = easy.transfer();

        transfer
            .write_function(|data| {
                buffer.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();

        transfer.perform().unwrap_or_default();
    }

    String::from_utf8_lossy(&buffer).to_string()
}
