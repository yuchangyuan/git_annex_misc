use std::io;
use git_annex_misc::backend::GitAnnexBackend;

struct XMailDirBackend {}

impl GitAnnexBackend for XMailDirBackend {
    fn new() -> XMailDirBackend {
        XMailDirBackend {}
    }

    fn version(&self) -> usize { 1 }
    fn can_verify(&self) -> bool { true }
    fn is_stable(&self) -> bool { true }
    fn is_cryptographically_secure(&self) -> bool { false }

    fn name(&self) -> &'static str{ "XMAILDIR" }

    fn genkey(&self, file: &str, size: u64) -> io::Result<String> {
        let mut stem: &str = file;

        // strip anything before '/'
        if let Some(x) = file.rfind('/') {
            stem = &file[(x+1)..]
        }

        // strip anything after ','
        if let Some(x) = stem.find(',') {
            stem = &stem[..x]
        }

        Ok(format!("{}-s{}--{}", self.name(), size, stem))
    }
}

fn main() {
    let backend: XMailDirBackend = GitAnnexBackend::new();
    backend.main_loop().unwrap()
}
