use std::fs::File;
use std::io::Write;
use sha2::{Sha256, Digest};
use sha3::Keccak256;

fn main() {
    let random_seed = "RANDOM_SEED".as_bytes();

    // SHA256 (SHA2)
    for i in 1..15 {
        let random_bytes = create_random_bytes_from_seed(random_seed, 2usize.pow(i as u32) as usize);
        let hash = sha256(&random_bytes);
        dump_bytes_to_file(&random_bytes, &format!("testoutput/sha256/{}/input", 2usize.pow(i as u32) as usize)).unwrap();
        dump_bytes_to_file(&hash, &format!("testoutput/sha256/{}/output", 2usize.pow(i as u32) as usize)).unwrap();
    }

    // KECCAK256 (SHA3)
    for i in 1..15 {
        let random_bytes = create_random_bytes_from_seed(random_seed, 2usize.pow(i as u32) as usize);
        let hash = keccak256(&random_bytes);
        dump_bytes_to_file(&random_bytes, &format!("testoutput/keccak256/{}/input", 2usize.pow(i as u32) as usize)).unwrap();
        dump_bytes_to_file(&hash, &format!("testoutput/keccak256/{}/output", 2usize.pow(i as u32) as usize)).unwrap();
    }

    // BLAKE3
    for i in 1..15 {
        let random_bytes = create_random_bytes_from_seed(random_seed, 2usize.pow(i as u32) as usize);
        let hash = blake3(&random_bytes);
        dump_bytes_to_file(&random_bytes, &format!("testoutput/blake3/{}/input", 2usize.pow(i as u32) as usize)).unwrap();
        dump_bytes_to_file(&hash, &format!("testoutput/blake3/{}/output", 2usize.pow(i as u32) as usize)).unwrap();
    }

    // BLAKE2B
    for i in 1..15 {
        let random_bytes = create_random_bytes_from_seed(random_seed, 2usize.pow(i as u32) as usize);
        let hash = blake2b(&random_bytes);
        dump_bytes_to_file(&random_bytes, &format!("testoutput/blake2b/{}/input", 2usize.pow(i as u32) as usize)).unwrap();
        dump_bytes_to_file(&hash, &format!("testoutput/blake2b/{}/output", 2usize.pow(i as u32) as usize)).unwrap();
    }
}

fn create_random_bytes_from_seed(seed: &[u8], size: usize) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(size);
    let mut hasher = Sha256::new();
    hasher.update(seed);
    let mut seed = hasher.finalize().to_vec();
    while bytes.len() < size {
        let mut hasher = Sha256::new();
        hasher.update(&seed);
        let hash = hasher.finalize();
        bytes.extend_from_slice(&hash);
        seed = hash.to_vec();
    }
    bytes.truncate(size);
    bytes
}

fn sha256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().into()
}

fn keccak256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(input);
    hasher.finalize().into()
}

fn blake2b(input: &[u8]) -> [u8; 64] {
    let mut hasher = blake2::Blake2b::new();
    hasher.update(input);
    hasher.finalize().into()
}

fn blake3(input: &[u8]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(input);
    hasher.finalize().into()
}

fn dump_bytes_to_file(bytes: &[u8], path: &str) -> std::io::Result<()> {
    // create intermediate directories in path if they don't exist
    std::fs::create_dir_all(std::path::Path::new(path).parent().unwrap())?;
    let mut file = File::create(path)?;
    file.write_all(hex::encode(bytes).as_bytes())
}