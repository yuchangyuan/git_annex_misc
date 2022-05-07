use std::io::{self, BufReader, Read};
use std::fs::File;

use git_annex_misc::backend::GitAnnexBackend;

use blake3;

struct XBlake3Backend {}

impl GitAnnexBackend for XBlake3Backend {
    fn new() -> XBlake3Backend {
        XBlake3Backend {}
    }

    fn version(&self) -> usize { 1 }
    fn can_verify(&self) -> bool { true }
    fn is_stable(&self) -> bool { true }
    fn is_cryptographically_secure(&self) -> bool { true }

    fn name(&self) -> &'static str{ "XBLAKE3E" }

    fn genkey(&self, path: &str, size: u64) -> io::Result<String> {
        // output progress
        let mut hasher = blake3::Hasher::new();

        let mut buf = [0u8; 1024*1024];
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut sz = 1;
        let mut sum = 0;

        while sz > 0 {
            sz = reader.read(&mut buf)?;
            sum += sz;
            hasher.update(&buf[0..sz]);
            println!("PROGRESS {}", sum)
        }

        let hash = hasher.finalize();

        Ok(format!("{}-s{}--{}", self.name(), size, hash.to_hex()))
    }
}

fn main() {
    let backend = XBlake3Backend {};
    backend.main_loop().unwrap()
}
