mod lib;

use std::env;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::ptr;
use std::mem;
use libc;

include!("lib/falcon.rs");

fn main() {
    let mut seed: [u8; 48] = [0; 48];
    let mut msg: [u8; 3300] = [0; 3300];
    let mut entropy_input: [u8; 48] = [0; 48];
    let mut m: *mut u8 = ptr::null_mut();
    let mut mlen: u64 = 0;
    let mut m1: *mut u8 = ptr::null_mut();
    let mut mlen1: u64 = 0;
    let mut sm: *mut u8 = ptr::null_mut();
    let mut smlen: u64 = 0;
    let mut count: i32 = 0;
    let mut done: i32 = 0;
    
    let mut pk: [u8; CRYPTO_PUBLICKEYBYTES as usize] = [0; CRYPTO_PUBLICKEYBYTES as usize];
    let mut sk: [u8; CRYPTO_SECRETKEYBYTES as usize] = [0; CRYPTO_SECRETKEYBYTES as usize];
    
    // Create the REQUEST file
    let fn_req = format!("PQCsignKAT_{}.req", CRYPTO_SECRETKEYBYTES);
    let mut fp_req = match OpenOptions::new().write(true).create(true).append(false).open(fn_req) {
        Err(why) => panic!("Couldn't open <PQCsignKAT_{}.req> for write: {}\n", CRYPTO_SECRETKEYBYTES, why),
        Ok(file) => file,
    };
    
    let fn_rsp = format!("PQCsignKAT_{}.rsp", CRYPTO_SECRETKEYBYTES);
    let mut fp_rsp = match OpenOptions::new().write(true).create(true).append(false).open(fn_rsp) {
        Err(why) => panic!("Couldn't open <PQCsignKAT_{}.rsp> for write: {}\n", CRYPTO_SECRETKEYBYTES, why),
        Ok(file) => file,
    };

    // Create random request
    for i in 0..48 {
        entropy_input[i] = i as u8;
    }

    unsafe {
        randombytes_init(entropy_input.as_mut_ptr(), ptr::null_mut(), 256);
    }

    for i in 0..5 {
        match fp_req.write(format!("count = {:<5}\n", i).as_bytes()) {
            Err(err) => panic!("Couldn't write: {}", err),
            Ok(n) => n, 
        };

        unsafe {
            randombytes(seed.as_mut_ptr(), 48);
        }
        lib::api::fprintBstr(&mut fp_req, ("seed = ").to_string(), seed.as_mut_ptr(), 48);

        mlen = 33 * (i + 1);
        fp_req.write(format!("mlen = {:<5}\n", mlen).as_bytes()).expect("Couldn't write");

        unsafe {
            randombytes(msg.as_mut_ptr(), mlen);
        }
        lib::api::fprintBstr(&mut fp_req, ("msg = ").to_string(), msg.as_mut_ptr(), mlen);
        fp_req.write(format!("pk = \n").as_bytes()).expect("Couldn't write");
        fp_req.write(format!("sk = \n").as_bytes()).expect("Couldn't write");
        fp_req.write(format!("smlen = \n").as_bytes()).expect("Couldn't write");
        fp_req.write(format!("sm = \n\n").as_bytes()).expect("Couldn't write");        
    }

    drop(fp_req);

    //Create the RESPONSE file based on what's in the REQUEST file
    let fn_req = format!("PQCsignKAT_{}.req", CRYPTO_SECRETKEYBYTES);
    let mut fp_req = match OpenOptions::new().read(true).open(fn_req) {
        Err(why) => panic!("\nCouldn't open <PQCsignKAT_{}.req> for read: {}\n", CRYPTO_SECRETKEYBYTES, why),
        Ok(file) => file,
    };
    
    fp_rsp.write(format!("# Falcon-512\n\n").as_bytes()).expect("\nCouldn't write response file\n");

    done = 0;    
    while done == 0 {
        let mut res = lib::api::FindMarker(&mut fp_req, ("count = ").to_string());
        if res > 0 {
            let mut num_line: [u8; 5] = [0; 5];
            fp_req.read(&mut num_line).unwrap();
            
            let res_str = String::from_utf8(num_line.to_vec()).unwrap();
            count = res_str.trim().parse::<i32>().unwrap();
        } else {
            done = 1;
            break;
        }

        fp_rsp.write(format!("count = {}\n", count).as_bytes()).expect("Couldn't write");
        
        res = lib::api::ReadHex(&mut fp_req, seed.as_mut_ptr(), 48, "seed = ".to_string());
        if res == 0 {
            println!("\nERROR: unable to read 'seed' from {}\n", res);
            return;
        }

        lib::api::fprintBstr(&mut fp_rsp, String::from("seed = "), seed.as_mut_ptr(), 48);
        
        unsafe { lib::api::randombytes_init(seed.as_mut_ptr(), ptr::null_mut(), 256); }
        
        res = lib::api::FindMarker(&mut fp_req, "mlen = ".to_string());
        if res > 0 {
            let mut num_line: [u8; 5] = [0; 5];
            fp_req.read(&mut num_line).unwrap();
            
            let res_str = String::from_utf8(num_line.to_vec()).unwrap();
            mlen = res_str.trim().parse::<u64>().unwrap();
        } else {
            println!("ERROR: unable to read 'mlen'\n");
            return;
        }
        
        fp_rsp.write(format!("mlen = {}\n", mlen).as_bytes()).expect("Couldn't write");
        
        unsafe {
            m = libc::calloc(mlen as usize, mem::size_of::<u8>()) as *mut u8;
            m1 = libc::calloc(mlen as usize, mem::size_of::<u8>()) as *mut u8;
            sm = libc::calloc((mlen as usize) + (CRYPTO_BYTES as usize), mem::size_of::<u8>()) as *mut u8;
        }

        res = lib::api::ReadHex(&mut fp_req, m, mlen as u32, String::from("msg = "));
        if res == 0 {
            println!("ERROR: unable to read 'msg' \n");
            return;
        }
        lib::api::fprintBstr(&mut fp_rsp, String::from("msg = "), m, mlen);
        
        // Generate the public/private keypair.
        let mut ret_val = lib::api::nist_sign_keypair(pk.as_mut_ptr(), sk.as_mut_ptr());
        if ret_val != 0 {
            println!("crypto_sign_keypair returned <{}>\n", ret_val);
            return;
        }
        lib::api::fprintBstr(&mut fp_rsp, String::from("pk = "), pk.as_mut_ptr(), CRYPTO_PUBLICKEYBYTES as u64);
        lib::api::fprintBstr(&mut fp_rsp, String::from("sk = "), sk.as_mut_ptr(), CRYPTO_SECRETKEYBYTES as u64);       
        
        // Sign
        ret_val = lib::api::nist_crypto_sign(sm, &mut smlen, m, mlen, sk.as_mut_ptr());
        if ret_val != 0 {
            println!("crypto_sign returned <{}>\n", ret_val);
            return;
        }
        fp_rsp.write(format!("smlen = {}\n", smlen).as_bytes()).expect("Couldn't write");
        lib::api::fprintBstr(&mut fp_rsp, String::from("sm = "), sm, smlen);
        fp_rsp.write(format!("\n").as_bytes()).expect("Couldn't write");
        
        // Verify
        ret_val = lib::api::nist_crypto_sign_open(m1, &mut mlen1, sm, smlen, pk.as_mut_ptr());
        if ret_val != 0 {
            println!("crypto_sign_open returned <{}>\n", ret_val);
            return;
        }
        
        if mlen != mlen1 {
            println!("crypto_sign_open returned bad 'mlen': Got <{}>, expected <{}>\n", mlen1, mlen);
            return;
        }
        
        // Compare m and m1
        let mut eqmem: bool = false;
        for i in 0..mlen {
            unsafe {
                if *m.offset(i as isize) != *m1.offset(i as isize) {
                    eqmem = true;
                    break;
                }
            }
        }
        if eqmem {
            println!("crypto_sign_open returned bad 'm' value\n");
            return;
        }
        
        unsafe { libc::free(m as *mut libc::c_void) };
        unsafe { libc::free(m1 as *mut libc::c_void) };
        unsafe { libc::free(sm as *mut libc::c_void) };
    };
    
    drop(fp_req);
    drop(fp_rsp);
}