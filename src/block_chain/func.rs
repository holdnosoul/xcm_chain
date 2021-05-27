
use chrono::prelude::*;
use sha256::{digest,digest_bytes,digest_file};
use std::path::Path;
use rand::Rng;

pub fn timestamp()->u64{
    let dt = Local::now();
    dt.timestamp_millis() as u64
}

pub fn sha256(input:&str)->String{
    digest(input)
}

#[warn(dead_code)]
pub fn sha256_byte(input:&[u8])->String{
    digest_bytes(input)
}

#[warn(dead_code)]
pub fn sha256_file(input:&str)->String{
    let path = Path::new(input);
    digest_file(path).unwrap()
}

pub fn rand_num()->String{
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    x.to_string()
}

pub fn private_key()->String{
    let s = format!("{}{}{}{}",sha256(&rand_num()),sha256(&timestamp().to_string()),rand_num(),timestamp().to_string());
    sha256(&s)
}



pub fn address(pk:&str)->String{
    let mut str = sha256(&format!("{}{}",sha256(&pk[0..pk.len()/2]),sha256(&pk[pk.len()/2..])));
    str.drain(32..);
    str
}

