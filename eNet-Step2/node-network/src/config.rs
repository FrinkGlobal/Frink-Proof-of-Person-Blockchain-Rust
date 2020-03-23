extern crate csv;
extern crate ini;

extern crate hex;

use ini::Ini;
use std::path::PathBuf;
use std::env;
use std::fs::File;
use std::fmt::{ self, Debug, Formatter };
use std::net::Ipv4Addr;

/*
 *  Declaration PeerInfo 
*/
pub struct PeerInfo {
    pub address: String,
    pub port: u16,
    pub key: Vec<u8>,
    pub connected: bool,
}

impl Debug for PeerInfo {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        let mut pk = String::new();
        for i in 0..20 {
            let tmp_str = format!("{:02X}", (&self).key[i]);
            pk.push_str(&tmp_str);
        }
        write!(f, "Peer information -> address : {}, port : {}, public key : {}, connected : {}", 
            self.address, self.port, pk, self.connected)
    }
}

/*
 *  Declaration HostInfo 
*/
pub struct HostInfo {
    pub port: u16,
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

impl Debug for HostInfo {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        let mut pk = String::new();
        for i in 0..10 {
            let tmp_str = format!("{:02X}", (&self).public_key[i]);
            pk.push_str(&tmp_str);
        }
        let mut sk = String::new();
        for i in 0..10 {
            let tmp_str = format!("{:02X}", (&self).private_key[i]);
            sk.push_str(&tmp_str);
        }
        write!(f, "\nHost information -> port : {}, public key : {}..., private key : {}...", 
            self.port, pk, sk)
    }
}

impl HostInfo {
    pub fn new() -> Self {
        HostInfo {
            port: 0,
            public_key: Vec::<u8>::new(),
            private_key: Vec::<u8>::new(),
        }
    }
}

/*
 *  Declaration functions to read and save for Config.ini
*/
#[allow(dead_code)]
pub fn get_hosts() -> Vec<HostInfo> {
    let mut conf_path = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("`CARGO_MANIFEST_DIR` is alway set by cargo.").to_string()
    );
    conf_path.pop();
    let conf_path = conf_path.join("config\\conf.ini");

    let conf = match Ini::load_from_file(conf_path) {
        Ok(v) => v,
        Err(_) => return Vec::<HostInfo>::new(),
    };
    
    let mut hosts = Vec::<HostInfo>::new();
    for (_sec, prop) in conf.iter() {        
        let host = HostInfo {
            port: (prop.get("port").unwrap()).parse::<u16>().unwrap(),
            public_key: hex::decode(prop.get("public").unwrap()).unwrap(),
            private_key: hex::decode(prop.get("private").unwrap()).unwrap(),
        };
        hosts.push(host);
    }
    
    return hosts;
}

#[allow(dead_code)]
pub fn set_hosts(hosts: &Vec<HostInfo>) -> bool {
    let mut conf_path = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("`CARGO_MANIFEST_DIR` is alway set by cargo.").to_string()
    );
    conf_path.pop();
    let conf_path = conf_path.join("config\\conf.ini");

    let mut conf = Ini::new();
    let ini_hosts = get_hosts();
    let mut index: u8 = 1;
    for host in ini_hosts {
        conf.with_section(Some(format!("Host {}", index)))
            .set("port", host.port.to_string())
            .set("public", hex::encode_upper(host.public_key.clone()))
            .set("private", hex::encode_upper(host.private_key.clone()));
        index = index + 1;
    }
    index = 1;
    for host in hosts {
        conf.with_section(Some(format!("Host {}", index)))
            .set("port", host.port.to_string())
            .set("public", hex::encode_upper(host.public_key.clone()))
            .set("private", hex::encode_upper(host.private_key.clone()));
        index = index + 1;
    }

    conf.write_to_file(conf_path).unwrap();

    return true;
}

#[allow(dead_code)]
pub fn set_host(host: &HostInfo, id: u16) -> bool {
    let mut conf_path = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("`CARGO_MANIFEST_DIR` is alway set by cargo.").to_string()
    );
    conf_path.pop();
    let conf_path = conf_path.join("config\\conf.ini");

    let mut conf = Ini::new();
    let ini_hosts = get_hosts();
    let mut index: u8 = 1;
    for host in ini_hosts {
        conf.with_section(Some(format!("Host {}", index)))
            .set("port", host.port.to_string())
            .set("public", hex::encode_upper(host.public_key.clone()))
            .set("private", hex::encode_upper(host.private_key.clone()));
        index = index + 1;
    }

    conf.with_section(Some(format!("Host {}", id + 1)))
        .set("port", host.port.to_string())
        .set("public", hex::encode_upper(host.public_key.clone()))
        .set("private", hex::encode_upper(host.private_key.clone()));
   
    conf.write_to_file(conf_path).unwrap();

    return true;
}

/*
 *  Declaration functions to read and save for Peerlist.csv
*/
#[allow(dead_code)]
pub fn read_peerlist() -> Vec<PeerInfo> {
        
    let mut _peerlist = Vec::<PeerInfo>::new();

    let mut csv_path = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("`CARGO_MANIFEST_DIR` is alway set by cargo.").to_string()
    );
    csv_path.pop();
    let csv_path = csv_path.join("config\\peerlist.csv");

    let fp = match File::open(csv_path) {
        Ok(v) => v,
        Err(_) => return _peerlist,
    };

    let mut csv_file = csv::ReaderBuilder::new()
                            .has_headers(false)
                            .delimiter(b',')
                            .from_reader(fp);

    for result in csv_file.records() {
        let record = result.expect("a CSV Record");
        let _peerdata = PeerInfo {
            address: ((&record[0]).to_string()).trim().to_string(),
            port: ((&record[1]).to_string()).trim().parse::<u16>().unwrap(),
            key: hex::decode((&record[2]).to_string().trim()).unwrap(),
            connected: false,
        };
        
        _peerlist.push(_peerdata);
    }

    _peerlist
}

#[allow(dead_code)]
pub fn save_peerlist(peers: &[PeerInfo]) -> bool {
    let mut csv_path = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("`CARGO_MANIFEST_DIR` is alway set by cargo.").to_string()
    );
    csv_path.pop();
    let csv_path = csv_path.join("config\\peerlist.csv");

    // let fp = match File::open(csv_path) {
    //     Ok(v) => v,
    //     Err(_) => return false,
    // };

    let mut csv_file = csv::WriterBuilder::new()
                            .has_headers(false)
                            .delimiter(b',')
                            .from_path(csv_path).unwrap();
    
    for peer in peers {
        let mut key_str = String::new();
        for i in 0..peer.key.len() {
            key_str.push_str(&format!("{:02X}", peer.key[i as usize]))
        }
        match csv_file.write_record(&[peer.address.to_string(), peer.port.to_string(), key_str]) {
            Ok(()) => continue,
            Err(_) => return false,
        }
    }
    match csv_file.flush() {
        Ok(()) => true,
        Err(_) => false,
    };

    return true;
}

#[allow(dead_code)]
pub fn parse_string_to_vec(ip_str: &String) -> Vec<u8> {
    
    let ip_addr: Vec<&str> = ip_str.trim().split(".").collect();
    if ip_addr.len() != 4 {
        return Vec::new();
    }
    
    let mut ip_vec = Vec::new();
    for i in 0..4 {
        let num: i32 = ip_addr[i].parse().unwrap();
        if num < 0 || num > 255 {
            return Vec::new();
        }

        ip_vec.push(num as u8);
    }

    return ip_vec; 
}

#[allow(dead_code)]
pub fn parse_string_to_ip(ip_str: &String) -> Ipv4Addr {
    
    let ip_vec: Vec<u8> = parse_string_to_vec(ip_str);
    if ip_vec.len() != 4 {
        return Ipv4Addr::UNSPECIFIED;
    }
    
    return Ipv4Addr::new(ip_vec[0], ip_vec[1], ip_vec[2], ip_vec[3]);
}

#[allow(dead_code)]
pub fn parse_string_to_reverse_ip(ip_str: &String) -> Ipv4Addr {
    
    let ip_vec: Vec<u8> = parse_string_to_vec(ip_str);
    if ip_vec.len() != 4 {
        return Ipv4Addr::UNSPECIFIED;
    }
    
    return Ipv4Addr::new(ip_vec[3], ip_vec[2], ip_vec[1], ip_vec[0]);
}