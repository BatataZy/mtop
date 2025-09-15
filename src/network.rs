use std::{net, str::FromStr};

use curl::easy::Easy;

use crate::{io::read, run, unit_types::Delta, BUFFER};

pub struct Adapter {
    pub name: String,
    pub interface: String,
    pub vendor: String,
    pub connection: String,
    pub local_ip: net::Ipv4Addr,
    pub public_ip: net::Ipv4Addr,
}

impl Adapter {
    const fn new() -> Self {
        Self {
            name: String::new(),
            interface: String::new(),
            vendor: String::new(),
            connection: String::new(),
            local_ip: net::Ipv4Addr::UNSPECIFIED,
            public_ip: net::Ipv4Addr::UNSPECIFIED,
        }
    }

    fn from_nmcli(stdout: &[u8]) -> Option<Self> {
        let attributes: [String; 5] = String::from_utf8_lossy(stdout)
            .split('\n')
            .map(|line| line.split_once(':').unwrap_or_default().1.to_owned())
            .next_chunk::<5>()
            .expect("there should always be at least 5 items");

        Some(Self {
            local_ip: net::Ipv4Addr::from_str(attributes[4].strip_suffix("/24")?)
                .expect("string should be a valid IPv4 address"),
            public_ip: net::Ipv4Addr::UNSPECIFIED,
            name: attributes[0].clone(),
            interface: attributes[1].clone(),
            vendor: attributes[2].clone(),
            connection: attributes[3].clone(),
        })
    }
}

pub struct Network {
    pub up: Delta,
    pub down: Delta,
    pub adapter: Adapter,
}

impl Network {
    pub fn new() -> Self {
        Self {
            up: Delta::new(),
            down: Delta::new(),
            adapter: Adapter::new(),
        }
    }

    pub fn update(&mut self) {
        let current_adapter = Adapter::from_nmcli(
            &run("nmcli -f GENERAL.DEVICE,GENERAL.TYPE,GENERAL.VENDOR,GENERAL.CONNECTION,IP4.ADDRESS -t device show").stdout
        ).unwrap_or(Adapter::new());

        if self.adapter.name != current_adapter.name
            || self.adapter.local_ip != current_adapter.local_ip
        {
            self.adapter = current_adapter;

            self.adapter.public_ip = net::Ipv4Addr::from_str(&curl("https://api.ipify.org"))
                .unwrap_or(net::Ipv4Addr::UNSPECIFIED);
        }

        self.up.add(
            read(
                &(format!("/sys/class/net/{}/statistics/tx_bytes", self.adapter.name)),
                0,
                0,
            )
            .parse::<u32>()
            .expect("fully numeric string")
                * 8
                / u32::from(BUFFER),
        );

        self.down.add(
            read(
                &(format!("/sys/class/net/{}/statistics/rx_bytes", self.adapter.name)),
                0,
                0,
            )
            .parse::<u32>()
            .expect("fully numeric string")
                * 8
                / u32::from(BUFFER),
        );
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
            .expect("writes properly onto buffer");

        transfer.perform().unwrap_or_default();
    }

    String::from_utf8_lossy(&buffer).to_string()
}
