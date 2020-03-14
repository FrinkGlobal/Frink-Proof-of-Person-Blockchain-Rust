extern crate enet;

use enet::*;
use std::fmt::{ self, Debug, Formatter };
use std::net::Ipv4Addr;

use secure_sign::NistCryptography;

mod config;
pub use config::PeerInfo;

// Declaratio of Constants
const DEFAULT_PORT: u16 = 8875;
const MAX_PEERS_COUNT: usize = 50;

// Declaratio of Class Node
pub struct Node {
    port: u16,
    secure: NistCryptography,
    peers: Vec<PeerInfo>,
}

impl Debug for Node {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        let mut seed = String::new();
        for i in 0..((&self).secure.seed.len()) {
            let tmp_str = format!("{:02X}", (&self).secure.seed[i]);
            seed.push_str(&tmp_str);
        }

        let mut pk = String::new();
        for i in 0..((&self).secure.public_key.len()) {
            let tmp_str = format!("{:02X}", (&self).secure.public_key[i]);
            pk.push_str(&tmp_str);
        }

        let mut sk = String::new();
        for i in 0..((&self).secure.private_key.len()) {
            let tmp_str = format!("{:02X}", (&self).secure.private_key[i]);
            sk.push_str(&tmp_str);
        }
        write!(f, "\nport : {}\nseed : {}\n\npublic key : {}\n\nprivate key : {}\n", 
            (&self).port, seed, pk, sk)
    }
}

impl Node {
    pub fn new(port: u16) -> Self {
        Node {
            port: match port {
                0 => DEFAULT_PORT,
                _ => port,
            },
            secure: NistCryptography::new(),            
            peers: Vec::<PeerInfo>::new()
        }
    }

    pub fn init(&mut self) {
        &self.secure.init();
        &self.get_peerlist();
    }

    pub fn get_peerlist(&mut self) {
        &self.peers.clear();
        &self.peers.append(&mut (config::read_peerlist()));
    }

    pub fn start_service(&mut self, enet: &mut Enet) {
        let local_addr = Address::new(Ipv4Addr::LOCALHOST, (&self).port);

        let mut host = enet
            .create_host::<()>(
                Some(&local_addr),
                MAX_PEERS_COUNT,
                ChannelLimit::Maximum,
                BandwidthLimit::Unlimited,
                BandwidthLimit::Unlimited,
            )
        .expect("could not create host");

        loop {
            match host.service(1000).expect("service failed") {
                Some(Event::Connect(_)) => println!("new connection!"),
                Some(Event::Disconnect(..)) => println!("disconnect!"),
                Some(Event::Receive {
                    channel_id,
                    ref packet,
                    ..
                }) => println!("got packet on channel {}, content: '{}'", channel_id,
                             std::str::from_utf8(packet.data()).unwrap()),
                _ => (),
            }

            println!("node {}", (&self).port);
        }
    }
}