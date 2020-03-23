#[allow(unused_assignments)]

extern crate enet;
extern crate hex;
extern crate chrono;

use enet::*;
use std::fmt::{ self, Debug, Formatter };
use std::net::Ipv4Addr;
use libc;
use std::mem;
use chrono::prelude::*;

use secure_sign::NistCryptography;
use secure_sign::CRYPTO_BYTES;

mod config;
pub use config::PeerInfo;
pub use config::HostInfo;
pub use config::get_hosts;
pub use config::set_hosts;
pub use config::read_peerlist;
pub use config::save_peerlist;
pub use config::parse_string_to_vec;
pub use config::parse_string_to_ip;
pub use config::parse_string_to_reverse_ip;
pub use secure_sign::randombytes;

/* 
 *  Declaratio of Constants
 */
const DEFAULT_PORT: u16 = 8875;
const MAX_PEERS_COUNT: usize = 50;

/* 
 *  Declaratio of Receive Buffer
 */
pub struct RecvMsg {
    pub timestamp: String,
    pub sender: u16,
    pub msg: Vec<u8>,
}

impl RecvMsg {
    pub fn new() -> Self {
        RecvMsg {
            timestamp: String::new(),
            sender: 0,
            msg: Vec::<u8>::new(),
        }
    }
}

/* 
 *  Declaratio of Class HostRepo
 */
pub struct HostRepo {
    pub port: u16,
    pub host: Vec<Host<()>>,
    pub peers: Vec<PeerInfo>,
    pub secure: NistCryptography,
    pub recv_messages: Vec<RecvMsg>,
    pub received: bool,
}

impl Debug for HostRepo {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        let mut seed = String::new();
        for i in 0..((&self).secure.seed.len()) {
            let tmp_str = format!("{:02X}", (&self).secure.seed[i]);
            seed.push_str(&tmp_str);
        }

        let mut pk = String::new();
        for i in 0..20 {
            let tmp_str = format!("{:02X}", (&self).secure.public_key[i]);
            pk.push_str(&tmp_str);
        }

        let mut sk = String::new();
        for i in 0..20 {
            let tmp_str = format!("{:02X}", (&self).secure.private_key[i]);
            sk.push_str(&tmp_str);
        }
        write!(f, "Host -> port : {}, peers : {}\nseed : {}...\npublic key : {}...\nprivate key : {}...\n", 
            self.port, self.peers.len(), seed, pk, sk)
    }
}

impl HostRepo {
    pub fn new(port: u16) -> Self {
        HostRepo {
            port: match port {
                0 => DEFAULT_PORT,
                _ => port,
            },
            host: Vec::<Host<()>>::new(),
            peers: Vec::<PeerInfo>::new(),
            secure: NistCryptography::new(),
            recv_messages: {
                let recv_msg = RecvMsg::new();
                let mut recv_vec = Vec::<RecvMsg>::new();
                recv_vec.push(recv_msg);
                recv_vec
            },
            received: false,
        }
    }

    pub fn init(&mut self) -> bool {
        self.secure.init();
        self.recv_messages.clear();
        self.received = false;

        if !(&self.read_peerlist()) {
            println!("Read Fail!!!! from {}", self.port);
            return false;
        }
        if !(&self.connect_peers()) {
            return false;
        }

        return true;
    }
    
    pub fn same_address(&self, addr: &String, port: &u16) -> bool {
        if (addr.trim().parse() == Ok(Ipv4Addr::LOCALHOST)) && (*port == (&self).port) {
            return true;
        } else {
            return false;
        }
    }

    pub fn read_peerlist(&mut self) -> bool {
        &self.peers.clear();
        &self.peers.append(&mut (config::read_peerlist()));

        match self.peers.len() {
            0 => false, 
            _ => true,
        }
    }

    pub fn save_peerlist(&mut self) -> bool {
        config::save_peerlist(&self.peers)
    }
    
    pub fn add_peer_info(&mut self, peer_info: PeerInfo) {
        self.peers.push(peer_info);
    }

    #[allow(dead_code)]
    pub fn connect_peer(&mut self, peer_info: PeerInfo) -> bool {
        let peer_address = peer_info.address.clone();
        let peer_port = peer_info.port;

        if self.host.len() > 0 {
            if self.same_address(&peer_address, &peer_port) {
                return false;
            }

            let peer_host = &mut self.host[0];
            if !peer_info.connected {
                peer_host.connect(&Address::new(peer_address.parse().unwrap(), peer_port), MAX_PEERS_COUNT, 0)
                    .expect("Connect failed");
            }
        } else {
            return false;
        }

        self.add_peer_info(peer_info);

        return true;
    }

    pub fn connect_peers(&mut self) -> bool {
        if self.peers.len() == 0 {
            return false;
        }

        for i in 0..self.peers.len() {
            let peer_address = self.peers[i].address.clone();
            let peer_port = self.peers[i].port;

            
            if self.same_address(&peer_address, &peer_port) {
                continue;
            }
            
            if self.host.len() > 0 {
                let peer_host = &mut self.host[0];

                if !self.peers[i].connected {
                    match peer_host.connect(&Address::new(peer_address.trim().parse().unwrap(), peer_port), MAX_PEERS_COUNT, 0) {
                        Ok(_) => continue,
                        Err(_) => return false,
                    }
                }
            }
        }

        return true;
    }

    pub fn generate_keypair(&mut self) -> bool {
        let res = self.secure.generate_keypair();        
        match res {
            0 => true, 
            _ => false,
        }
    }

    #[allow(unused_assignments)]
    pub fn sign_message(&mut self, mut _msg: &Vec<u8>) -> Vec<u8> {
        
        let mut smsg = Vec::new();
        
        let mut m = std::ptr::null_mut();
        let mlen = _msg.len();
        let mut sm: *mut u8 = std::ptr::null_mut();
        let mut smlen: u64 = 0;        

        unsafe {
            m = libc::calloc(mlen as usize, mem::size_of::<u8>()) as *mut u8;
            sm = libc::calloc((mlen as usize) + (CRYPTO_BYTES as usize), mem::size_of::<u8>()) as *mut u8;
        }
        
        unsafe {
            for i in 0..mlen {
                *m.offset(i as isize) = _msg[i];
            }
        }
            
        // Sign
        let ret_val = self.secure.sign_msg(sm, &mut smlen, m, mlen as u64);

        if ret_val > 0 {
            println!("Failed to sign message...", );
        }

        // Return Sign message
        unsafe {
            for i in 0..smlen {
                smsg.push(*sm.offset(i as isize));
            }
        }

        unsafe { libc::free(m as *mut libc::c_void) };
        unsafe { libc::free(sm as *mut libc::c_void) };

        return smsg;
    }

    #[allow(unused_assignments)]
    pub fn sign_message_foriegn_key(&mut self, mut _msg: &Vec<u8>, pk: &mut Vec<u8>) -> Vec<u8> {
        
        let mut smsg = Vec::new();
        
        let mut m = std::ptr::null_mut();
        let mlen = _msg.len();
        let mut sm: *mut u8 = std::ptr::null_mut();
        let mut smlen: u64 = 0;        

        unsafe {
            m = libc::calloc(mlen as usize, mem::size_of::<u8>()) as *mut u8;
            sm = libc::calloc((mlen as usize) + (CRYPTO_BYTES as usize), mem::size_of::<u8>()) as *mut u8;
        }
        
        unsafe {
            for i in 0..mlen {
                *m.offset(i as isize) = _msg[i];
            }
        }
            
        // Sign
        let ret_val = self.secure.sign_foreign_key(sm, &mut smlen, m, mlen as u64, pk.as_mut_ptr());

        if ret_val > 0 {
            println!("Failed to sign message...", );
        }

        // Return Sign message
        unsafe {
            for i in 0..smlen {
                smsg.push(*sm.offset(i as isize));
            }
        }

        unsafe { libc::free(m as *mut libc::c_void) };
        unsafe { libc::free(sm as *mut libc::c_void) };

        return smsg;
    }

    pub fn verify_message(&mut self, mut _msg: &Vec<u8>) -> Vec<u8> {
        
        let mut vmsg = Vec::new();

        #[allow(unused_assignments)]
        let mut sm = std::ptr::null_mut();
        let smlen = _msg.len();
        #[allow(unused_assignments)]
        let mut vm = std::ptr::null_mut();
        let mut vmlen: u64 = 0;

        unsafe {
            sm = libc::calloc(smlen as usize, mem::size_of::<u8>()) as *mut u8;
            vm = libc::calloc(smlen as usize, mem::size_of::<u8>()) as *mut u8;
        }
        
        unsafe {
            for i in 0..smlen {
                *sm.offset(i as isize) = _msg[i];
            }
        }
            
        // Verify
        let ret_val = self.secure.verify_msg(vm, &mut vmlen, sm, smlen as u64);
        if ret_val != 0 {
            println!("Fail to verify message with public key", );
        }

        // Return Verify string
        unsafe {
            for i in 0..vmlen {
                vmsg.push(*vm.offset(i as isize));
            }
        }

        unsafe { libc::free(sm as *mut libc::c_void) };
        unsafe { libc::free(vm as *mut libc::c_void) };

        return vmsg;
    }

    pub fn verify_message_foriegn_key(&mut self, mut _msg: &Vec<u8>, sk: &mut Vec<u8>) -> Vec<u8> {
        
        let mut vmsg = Vec::new();
        #[allow(unused_assignments)]
        let mut sm = std::ptr::null_mut();
        let smlen = _msg.len();
        #[allow(unused_assignments)]
        let mut vm = std::ptr::null_mut();
        let mut vmlen: u64 = 0;

        unsafe {
            sm = libc::calloc(smlen as usize, mem::size_of::<u8>()) as *mut u8;
            vm = libc::calloc(smlen as usize, mem::size_of::<u8>()) as *mut u8;
        }
        
        unsafe {
            for i in 0..smlen {
                *sm.offset(i as isize) = _msg[i];
            }
        }
            
        // Verify
        let ret_val = self.secure.verify_foreign_key(vm, &mut vmlen, sm, smlen as u64, sk.as_mut_ptr());
        if ret_val != 0 {
            println!("\nFali to verify with foreign key", );
        }

        // Return Verify string
        unsafe {
            for i in 0..vmlen {
                vmsg.push(*vm.offset(i as isize));
            }
        }

        unsafe { libc::free(sm as *mut libc::c_void) };
        unsafe { libc::free(vm as *mut libc::c_void) };

        return vmsg;
    }

    pub fn broadcast_message(&mut self, _msg: &Vec<u8>) {
        let sign_msg = self.sign_message(_msg);
        if sign_msg.len() == 0 {
            println!("Couldn't sign message...", );
            return;
        }

        let peer_host = &mut self.host[0];
        let mut addresses = Vec::<Ipv4Addr>::new();
        let mut ports = Vec::<u16>::new();
        for mut _peer in peer_host.peers() {
            if !_peer.address().ip().is_unspecified() {
                
                let mut is_exist = false;
                for i in 0..ports.len() {
                    if (addresses[i] == *_peer.address().ip()) 
                    && (ports[i] == _peer.address().port()) {
                        is_exist = true;
                    }                    
                }
                if !is_exist {
                    let _pack_res = _peer.send_packet(
                        Packet::new(sign_msg.as_slice(), PacketMode::ReliableSequenced).unwrap(),
                        1,
                    );
                    addresses.push(*_peer.address().ip());
                    ports.push(_peer.address().port());
                }
            }
        }
    }

    pub fn send_message(&mut self, _peer: &mut Peer<()>, _msg: &Vec<u8>) -> Result<(), Error> {        
        return _peer.send_packet(
            Packet::new(_msg.as_slice(), PacketMode::ReliableSequenced).unwrap(),
            1,
        );
    }

    fn find_peer(&self, addr: Ipv4Addr, port: u16) -> i32 {
        let mut index: i32 = -1;
        for i in 0..self.peers.len() {
            let peer_info = &self.peers[i];
            if (parse_string_to_reverse_ip(&peer_info.address) == addr) 
            && (peer_info.port == port) {
                index = i as i32;
                break;
            } 
        }

        return index;
    }

    pub fn is_found_peer(&mut self, peer_info: &PeerInfo) -> bool {
        let mut res = false;
        let peer_host = &mut self.host[0];
        for p in peer_host.peers() {
            if (peer_info.address == p.address().ip().to_string()) && (peer_info.port == p.address().port()) {
                res = true;
                break;
            }
        }
        return res;
    }

    pub fn process_message(&mut self, data: Vec<u8>, addr: Ipv4Addr, port: u16) -> bool {
        let _data_len = data.len();
        
        print!("Got packet on {} from {}:{}", self.port, addr.to_string(), port);

        // Find peer public key
        let index = self.find_peer(addr, port);
        if index < 0 {
            println!("\nCouldn't find received peer", );
            return false;
        }

        // Get the public key of receiver
        let mut pk = Vec::new();
        for i in 0..self.peers[index as usize].key.len() {
            pk.push(self.peers[index as usize].key[i]);
        }
        
        let vmsg = self.verify_message_foriegn_key(&data, &mut pk);

        // Save receive data
        let mut recv_msg = RecvMsg::new();
        let localtime: DateTime<Local> = Local::now();

        recv_msg.sender = port;
        recv_msg.timestamp.push_str(&localtime.format("%Y-%m-%d %H:%M:%S").to_string());

        for i in 0..vmsg.len() {
            recv_msg.msg.push(vmsg[i]);
        }

        self.recv_messages.push(recv_msg);

        self.received = true;

        print!("\nVerified message -> {} : ", &localtime.format("%Y-%m-%d %H:%M:%S").to_string());

        for i in 0..vmsg.len() {
            print!("{:02X}", vmsg[i]);
        }
        print!("\n", );

        if vmsg.len() == 0 {
            false
        } else {
            true
        }
    }

    pub fn execute(&mut self) {
        let peer_host = &mut self.host[0];
        let mut data: Vec<u8> = Vec::new();
        let mut recv_port: u16 = 0;
        let mut recv_address: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
        let mut sender_peer = Vec::new();

        match peer_host.service(10).expect("service failed") {
            Some(Event::Connect(ref p)) => {
                for peer in &mut self.peers {
                    let ip_addr = parse_string_to_reverse_ip(&peer.address);
                    if (ip_addr == *(p.address().ip())) && (peer.port == p.address().port()) {
                        peer.connected = true;
                        break;
                    }
                }
            },
            Some(Event::Disconnect(ref p, _dt)) => {
                for peer in &mut self.peers {
                    if (peer.address == p.address().ip().to_string()) && (peer.port == p.address().port()) {
                        peer.connected = false;
                        break;
                    }
                }
            },
            Some(Event::Receive {
                ref sender,
                ref packet,
                channel_id: _,
            }) => {
                recv_port = *(&sender.address().port());
                recv_address = *(sender.address().ip());
                let pkdata = packet.data();
                for i in 0..pkdata.len() {
                    data.push(pkdata[i]);
                }
                sender_peer.push(sender);
            },
            _ => (),
        }

        if data.len() > 0 {
            let res = self.process_message(data, recv_address, recv_port);
            if !res {
                println!("Fail to process message", );
            }
        }
    }
}

/* 
 *  Declaratio of Class Node
 */
pub struct Node {
    pub net: Enet,
    pub hosts: Vec<HostRepo>,
}

impl Debug for Node {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\nhost count : {}\n", self.hosts.len())
    }
}

impl Node {
    pub fn new() -> Self {
        Node {
            net: Enet::new().expect("could not initialize ENet"),
            hosts: Vec::<HostRepo>::new(),
        }
    }

    pub fn init(&mut self) {
        self.hosts.clear();
    }

    pub fn create_host(&mut self, port: u16) {
        // Create a HostRepo instance
        let mut host_repo = HostRepo::new(port);
        
        // Get local address: 127.0.0.1:{port}
        let local_addr = Address::new(Ipv4Addr::LOCALHOST, port);

        // Create a enet::Host
        let host = (&self).net
            .create_host::<()>(
                Some(&local_addr),
                MAX_PEERS_COUNT,
                ChannelLimit::Maximum,
                BandwidthLimit::Unlimited,
                BandwidthLimit::Unlimited,
            )
            .expect("could not create host");
        
        // Initialize a HostRepo instance
        host_repo.host.clear();
        host_repo.host.push(host);

        // Save HostRepo information in Node network
        self.hosts.push(host_repo);
    }

    pub fn read_hosts(&mut self) -> bool {
        // Read Config.ini file
        let hosts_info: Vec<HostInfo> = config::get_hosts();
        if hosts_info.len() == 0 {
            return false;
        }
        
        // Construct Node network in local machine
        for host_info in &hosts_info {
            self.create_host(host_info.port);

            for j in 0..host_info.public_key.len() {
                if let Some(host) = self.hosts.last_mut() {
                    (*host).secure.public_key[j] = host_info.public_key[j];
                } else {
                    return false;
                }
            }
            for j in 0..host_info.private_key.len() {
                if let Some(host) = self.hosts.last_mut() {
                    (*host).secure.private_key[j] = host_info.private_key[j];
                } else {
                    return false;
                }
            }
        }

        return true;
    }

    pub fn save_hosts(&mut self) {
        let mut hosts_info: Vec<HostInfo> = Vec::<HostInfo>::new();

        // Save the key pair
        for i in 0..self.hosts.len() {
            let host = &self.hosts[i];
            let mut host_info = HostInfo::new();
            host_info.port = host.port;
            host_info.public_key = host.secure.public_key.iter().cloned().collect();
            host_info.private_key = host.secure.private_key.iter().cloned().collect();

            hosts_info.push(host_info);
        }
        config::set_hosts(&hosts_info);
    }

    pub fn save_host(&mut self, id: u16) {
        // Save the key pair
        if (id as usize) < self.hosts.len() {
            let host = &self.hosts[id as usize];
            let mut host_info = HostInfo::new();
            host_info.port = host.port;
            host_info.public_key = host.secure.public_key.iter().cloned().collect();
            host_info.private_key = host.secure.private_key.iter().cloned().collect();
 
            config::set_host(&host_info, id);
        }
    }

    pub fn execute(&mut self) {
        for i in 0..self.hosts.len() {
            let peer_host = &mut self.hosts[i];
            peer_host.execute();
        }
    }

    pub fn start(&mut self) {
        // Read conf file
        if !self.read_hosts() {
            println!("Couldn't read conf.ini file", );
            return;
        }

        // Initialize hosts of node
        for host in &mut self.hosts {
            if !(&host.init()) {
                println!("Couldn't read peerlist.csv file", );
                return;
            }
        }

        #[allow(unused_variables)]
        for i in 0..10 {
            self.execute();
        }
    }

    pub fn restart(&mut self) {
        self.init();
        self.start();
    }
}