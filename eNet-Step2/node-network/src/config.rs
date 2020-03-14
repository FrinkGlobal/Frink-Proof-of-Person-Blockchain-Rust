extern crate csv;
extern crate ini;

use ini::Ini;
use std::path::PathBuf;
use std::env;
use std::fs::File;

#[derive(Debug)]
pub struct PeerInfo {
    address: String,
    port: u16,
}

#[allow(dead_code)]
pub fn get_user_port() -> u16 {
    let mut conf_path = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("`CARGO_MANIFEST_DIR` is alway set by cargo.").to_string()
    );
    conf_path.pop();
    let conf_path = conf_path.join("config\\conf.ini");

    let conf = Ini::load_from_file(conf_path).unwrap();
    let section = conf.section(Some("User")).unwrap();
    let port = section.get("port").unwrap();
    
    return port.parse::<u16>().unwrap();
}

#[allow(dead_code)]
pub fn set_user_port(_port: u16) -> bool {
    return true;
}

#[allow(dead_code)]
pub fn read_peerlist() -> Vec<PeerInfo> {
        
    let mut csv_path = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("`CARGO_MANIFEST_DIR` is alway set by cargo.").to_string()
    );
    csv_path.pop();
    let csv_path = csv_path.join("config\\peerlist.csv");

    let fp = File::open(csv_path).expect("Couldn't open csv file.\n");

    let mut csv_file = csv::ReaderBuilder::new()
                            .has_headers(false)
                            .delimiter(b',')
                            .from_reader(fp);

    let mut _peerlist = Vec::<PeerInfo>::new();

    for result in csv_file.records() {
        let record = result.expect("a CSV Record");
        
        let _peerdata = PeerInfo {
            address: ((&record[0]).to_string()).trim().to_string(),
            port: ((&record[1]).to_string()).trim().parse::<u16>().unwrap(),
        };
        
        _peerlist.push(_peerdata);
    }

    _peerlist
}

#[allow(dead_code)]
pub fn save_peerlist(_peers: &[PeerInfo]) -> bool {
    return true;
}