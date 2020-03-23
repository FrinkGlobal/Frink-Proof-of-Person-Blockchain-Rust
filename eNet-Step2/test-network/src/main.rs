#[allow(unused_imports)]
extern crate hex;

use libc;
use std::mem;

use node_network;
use node_network::Node;
use secure_sign::NistCryptography;

#[allow(unused_assignments)]
fn main() {

    test_net();

    // test_secure();
    
    println!("Bye! End.", );
}

#[allow(dead_code)]
fn test_net() {
    /* Initiate the Node Local Net */
    let mut node: Node = Node::new();
    node.init();

    // Read conf file
    if !node.read_hosts() {
        println!("Couldn't read conf.ini file", );
        return;
    }

    for host in &mut node.hosts {
        if !(&host.init()) {
            println!("Couldn't read peerlist.csv file", );
            return;
        }
    }
    
    // Generate random data
    let mut mlen: u64 = 20;
    let mut msg_str = Vec::new();
    
    if false {
        let mut msg = std::ptr::null_mut();

        unsafe {
            msg = libc::calloc(mlen as usize, mem::size_of::<u8>()) as *mut u8;        
            node_network::randombytes(msg, mlen);

            // Display message
            for i in 0..mlen {
                msg_str.push(*msg.offset(i as isize));
            }
        }
        unsafe { libc::free(msg as *mut libc::c_void) };
    } else {
        msg_str = Vec::from("Hello World");
        mlen = msg_str.len() as u64;        
    }

    // Execute hosts of node
    let mut i = 0;
    loop {
        node.execute();

        i += 1;
        if i > 20 {
            break;
        }
    }

    node.hosts[1].broadcast_message(&msg_str);
    loop {
        node.execute();

        i += 1;
        if i > 100 {
            break;
        }
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn test_secure() {
    
    let mut _secure: NistCryptography = NistCryptography::new();

    _secure.init();
    _secure.generate_keypair();

    print!("public key[{}]: ", _secure.public_key.len());

    for i in 0..20 {
        print!("{:02X}", _secure.public_key[i]);
    }

    print!("\nprivate key[{}] : ", _secure.private_key.len());

    for i in 0..20 {
        print!("{:02X}", _secure.private_key[i]);
    }
    print!("\n", );

    
    let msg_str = Vec::from("Hello World");
    let mlen = msg_str.len();
    let mut smlen = 100;
    let mut vmlen = 100;

    let mut m = std::ptr::null_mut();
    let mut vm = std::ptr::null_mut();
    let mut sm = std::ptr::null_mut();

    unsafe {
        m = libc::calloc(mlen as usize, mem::size_of::<u8>()) as *mut u8;
        sm = libc::calloc((mlen as usize) + (secure_sign::CRYPTO_BYTES as usize), mem::size_of::<u8>()) as *mut u8;
        vm = libc::calloc((mlen as usize) + (secure_sign::CRYPTO_BYTES as usize), mem::size_of::<u8>()) as *mut u8;
    }

    unsafe {
        for i in 0..mlen {
            *m.offset(i as isize) = msg_str[i];
        }
    }

    let mut pk = Vec::new();
    for i in 0.._secure.public_key.len() {
        pk.push(_secure.public_key[i]);
    }
    let mut sk = Vec::new();
    for i in 0.._secure.private_key.len() {
        sk.push(_secure.private_key[i]);
    }
    let ret = _secure.sign_foreign_key(sm, &mut smlen, m, mlen as u64, sk.as_mut_ptr());

    // let ret = _secure.verify_foreign_key(vm, &mut vmlen, sm, smlen, pk.as_mut_ptr());
    
    let mut smsg_str = String::from("");
    unsafe {
        print!("\n", );
        for i in 0..smlen {
            smsg_str.push_str(&format!("{:02X}", *sm.offset(i as isize)));
        }
    }

    let mut vmsg_str = String::from("");
    unsafe {
        for i in 0..vmlen {
            vmsg_str.push_str(&format!("{:02X}", *vm.offset(i as isize)));
        }
    }
    
    println!("sign return -> {}", ret);

    // println!("Message ({})-> {}", mlen, msg_str);
    println!("Signed Message ({})-> {}", smlen, smsg_str);
    println!("Verified Message ({})-> {}", vmlen, vmsg_str);

    unsafe { libc::free(m as *mut libc::c_void) };
    unsafe { libc::free(vm as *mut libc::c_void) };
    unsafe { libc::free(sm as *mut libc::c_void) };
    
}