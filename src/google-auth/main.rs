#![deny(warnings)]
#![allow(dead_code)]


use data_encoding::BASE32;
use std::time::{SystemTime, UNIX_EPOCH};
use hmacsha1::hmac_sha1;
use serde_derive::Deserialize;
use std::fs;

fn get_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[derive(Debug, Deserialize)]
struct GoogleAuthConfig {
    token: String
}

#[derive(Debug, Deserialize)]
struct WorkflowConfig {
    google_auth: Option<GoogleAuthConfig>
}


fn main() {
    let path = "~/.config/workflow/config.toml";
    let result = fs::read_to_string(path).unwrap();
    let decoded: WorkflowConfig = toml::from_str(result.as_str()).unwrap();
    let num = google_auth(decoded.google_auth.unwrap().token.as_bytes());
    println!("{:06}", num);
}

fn google_auth(token: &[u8]) -> u32{
    let mut secret = vec![0; BASE32.decode_len(token.len()).unwrap()];
    BASE32.decode_mut(token, &mut secret).unwrap();

    let secret = &secret[..];

    let input = get_epoch()/30;
    let mut hash = hmac_sha1(secret, &input.to_be_bytes());

    let index:usize = (hash[hash.len()-1] & 0x0f) as usize;
    let hash_part_tmp = & mut hash[index..index+4];
    hash_part_tmp[0] = hash_part_tmp[0] & 0x7F;

    let hash_part:[u8;4] = [hash_part_tmp[0], hash_part_tmp[1], hash_part_tmp[2], hash_part_tmp[3]];
    let num = u32::from_be_bytes(hash_part) % 1000000;

    // print!("hash: {:?}\n", hash);
    // print!("index: {:?}\n", index);
    // print!("hash_part: {:?}\n", hash_part);
    // print!("num: {:?}\n", num);
    num
}
