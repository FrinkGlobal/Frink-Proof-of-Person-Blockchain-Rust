#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

extern crate rand;

use std::fmt::{ self, Debug, Formatter };
use std::io::Write;
use std::ptr;
use rand::Rng;

include!("falcon.rs");

const SEEDBYTES: usize = 48;

pub struct NistCryptography {
    pub seed: [u8; SEEDBYTES],
    pub private_key: [u8; CRYPTO_SECRETKEYBYTES as usize],
    pub public_key: [u8; CRYPTO_PUBLICKEYBYTES as usize],
}

impl Debug for NistCryptography {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        let mut seed = String::new();
        for i in 0..((&self).seed.len()) {
            let tmp_str = format!("{:02X}", (&self).seed[i]);
            seed.push_str(&tmp_str);
        }

        let mut pk = String::new();
        for i in 0..((&self).public_key.len()) {
            let tmp_str = format!("{:02X}", (&self).public_key[i]);
            pk.push_str(&tmp_str);
        }

        let mut sk = String::new();
        for i in 0..((&self).private_key.len()) {
            let tmp_str = format!("{:02X}", (&self).private_key[i]);
            sk.push_str(&tmp_str);
        }
        write!(f, "\nseed : {}\n\npublic key : {}\n\nprivate key : {}\n", seed, pk, sk)
    }
}

impl NistCryptography {

    pub fn new() -> Self {
        NistCryptography {
            seed: [0; SEEDBYTES],
            private_key: [0; CRYPTO_SECRETKEYBYTES as usize],
            public_key: [0; CRYPTO_PUBLICKEYBYTES as usize],
        }
    }

    pub fn init(&mut self) {

        let mut entropy_input: [u8; SEEDBYTES] = [0; SEEDBYTES];

        // Create random request
        for i in 0..SEEDBYTES {
            entropy_input[i] = (rand::thread_rng().gen_range(0, SEEDBYTES)) as u8;
        }

        unsafe {
            randombytes_init(entropy_input.as_mut_ptr(), ptr::null_mut(), 256);
        }

        unsafe {
            randombytes(self.seed.as_mut_ptr(), 48);
        }
    }

    pub fn generate_keypair(&mut self) -> i32 {
        unsafe {
            let res = crypto_sign_keypair(self.public_key.as_mut_ptr(), self.private_key.as_mut_ptr());
            return res;
        }
    }

    pub fn sign_msg(&mut self, sm: *mut u8, smlen: &mut u64, m: *const u8, mlen: u64) -> i32 {
        unsafe {            
            let ret_val = crypto_sign(sm, smlen, m, mlen, self.private_key.as_mut_ptr());
            return ret_val;
        }
    }

    pub fn sign_foreign_key(&mut self, sm: *mut u8, smlen: *mut u64, m: *const u8, mlen: u64, _pk: *mut u8) -> i32 {
        unsafe {
            let ret_val = crypto_sign_public(sm, smlen, m, mlen, _pk);
            return ret_val;
        }
    }

    pub fn verify_msg(&mut self, m: *mut u8, mlen: *mut u64, sm: *const u8, smlen: u64) -> i32 {
        unsafe {
            let ret_val = crypto_sign_open(m, mlen, sm, smlen, self.public_key.as_mut_ptr());
            return ret_val;
        }
    }

    pub fn verify_foreign_key(&mut self, m: *mut u8, mlen: *mut u64, sm: *const u8, smlen: u64, _sk: *mut u8) -> i32 {
        unsafe {
            let ret_val = crypto_sign_open_private(m, mlen, sm, smlen, _sk);
            return ret_val;
        }
    }
}