#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader, SeekFrom};

include!("falcon.rs");

const MAX_MARKER_LEN: usize   = 50;

pub fn nist_sign_keypair(pk: *mut u8, sk: *mut u8) -> i32 {
    unsafe {
        let res = crypto_sign_keypair(pk, sk);
        return res;
    }    
}

pub fn nist_crypto_sign(sm: *mut u8, smlen: *mut u64, m: *const u8, mlen: u64, sk: *const u8) -> i32 {
    unsafe {
        let ret_val = crypto_sign(sm, smlen, m, mlen, sk);
        return ret_val;
    }
}

pub fn nist_crypto_sign_open(m: *mut u8, mlen: *mut u64, sm: *const u8, smlen: u64, pk: *const u8) -> i32 {
    unsafe {
        let ret_val = crypto_sign_open(m, mlen, sm, smlen, pk);
        return ret_val;
    }
}

pub fn fprintBstr(fp: &mut File, S: String, A: *mut u8, L: u64) {

    match fp.write(format!("{}", S).as_bytes()) {
        Err(err) => panic!("Couldn't write: {}", err),
        Ok(n) => n, 
    };

    unsafe {
        for i in 0..L {
            match fp.write(format!("{:02X}", *A.offset(i as isize)).as_bytes()) {
                Err(err) => panic!("Couldn't write: {}", err),
                Ok(n) => n, 
            };
        }
    }

    if L == 0 {
        match fp.write("00".as_bytes()) {
            Err(err) => panic!("Couldn't write: {}", err),
            Ok(n) => n, 
        };
    }

    match fp.write("\n".as_bytes()) {
        Err(err) => panic!("Couldn't write: {}", err),
        Ok(n) => n, 
    };
}

//
// ALLOW TO READ HEXADECIMAL ENTRY (KEYS, DATA, TEXT, etc.)
//
pub fn FindMarker(infile: &mut File, marker: String) -> i32 {
    let mut line: [u8; MAX_MARKER_LEN] = [0; MAX_MARKER_LEN];
    let mut buf: [u8; 1] = [0u8; 1];

	let mut len = marker.len();
	if len > (MAX_MARKER_LEN - 1) {
        len = MAX_MARKER_LEN-1;
    }

	for i in 0..len {
        let res = infile.read(&mut buf).expect("Couldn't read");
        line[i] = buf[0];

        if res == 0 {
            return 0;
        }
    }
    line[len] = b'\0';

    loop {
        let line_str = String::from_utf8((&line[0..len]).to_vec()).unwrap();

		if marker.as_str() == line_str {
            return 1;
        }

		for i in 0..(len - 1) {
            line[i] = line[i+1];
        }

        let res = infile.read(&mut buf).expect("Couldn't read");
		line[len-1] = buf[0];
        if res == 0 {
            return 0;
        }

        line[len] = b'\0';
	}

	// shouldn't get here
    return 0;
}

//
// ALLOW TO READ HEXADECIMAL ENTRY (KEYS, DATA, TEXT, etc.)
//
pub fn ReadHex(infile: &mut File, A: *mut u8, Length: u32, str: String) -> i32 {
    let mut started: i32 = 0;
    let mut ich: u8 = 0;
    let mut buf: [u8; 1] = [0u8; 1];

	if Length == 0 {
        unsafe {
            *A.offset(0) = 0x00;
            return 1;
        }
    }
    
    for i in 0..(Length as isize) {
        unsafe { *A.offset(i) = 0x00; }
    }

    started = 0;

    let res = FindMarker(infile, str);
	if res > 0 {
        while { match infile.read(&mut buf) {
            Err(_err) => false,
            Ok(n) => (n > 0)
        }} {
			if buf[0].is_ascii_hexdigit() == false {
				if started == 0 {
					if buf[0] == b'\n' {
                        break;
                    } else {
                        continue;
                    }
				} else {
                    break;
                }
            }
			started = 1;
			if (buf[0] >= b'0') && (buf[0] <= b'9') {
                ich = buf[0] - b'0';
            } else if (buf[0] >= b'A') && (buf[0] <= b'F') {
                ich = buf[0] - b'A' + 10;
            } else if (buf[0] >= b'a') && (buf[0] <= b'f') {
                ich = buf[0] - b'a' + 10;
            } else {
                // shouldn't ever get here
                ich = 0;
            }
			
			for i in 0..((Length - 1) as isize) {
                unsafe { *A.offset(i) = (*A.offset(i) << 4) | (*A.offset(i+1) >> 4); } 
            }
            unsafe { *A.offset((Length-1) as isize) = (*A.offset((Length-1) as isize) << 4) | ich; }
        }
    } else {
        return 0;
    }

	return 2;
}