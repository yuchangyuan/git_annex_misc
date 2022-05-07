use std::io;
use std::fs;
use std::os::unix::prelude::MetadataExt;

fn file_size(f: &str) -> io::Result<u64> {
    let metadata = fs::metadata(f)?;
    Ok(metadata.size())
}

fn to_yes_or_no(x:bool) -> &'static str {
    if x { "YES" } else { "NO" }
}

fn verify_fail() {
    println!("VERIFYKEYCONTENT-FAILURE");
}

fn protocol_error()
{
    println!("ERROR protocol error");
}

pub trait GitAnnexBackend {
    fn new() -> Self;

    fn version(&self) -> usize;
    fn can_verify(&self) -> bool;
    fn is_stable(&self) -> bool;
    fn is_cryptographically_secure(&self) -> bool;
    fn name(&self) -> &'static str;

    fn genkey(&self, file: &str, size: u64) -> io::Result<String>;


    fn line_process(&self, line: &str) {
        let args: Vec<&str> = line.trim().split(char::is_whitespace).collect();
        if args.len() < 1 {
            protocol_error();
            return;
        }

        //println!("args0 '{:?}'", args);
        match args[0] {
            "GETVERSION" => println!("VERSION {}", self.version()),
            "CANVERIFY" => println!("CANVERIFY-{}", to_yes_or_no(self.can_verify())),
            "ISSTABLE" => println!("ISSTABLE-{}", to_yes_or_no(self.is_stable())),
            "ISCRYPTOGRAPHICALLYSECURE" =>
                println!("ISCRYPTOGRAPHICALLYSECURE-{}", to_yes_or_no(self.is_cryptographically_secure())),
            "GENKEY" => {
                if args.len() < 2 {
                    protocol_error();
                    return;
                }

                let size;

                match file_size(args[1]) {
                    Ok(sz) => { size = sz },
                    Err(_) => {
                        println!("GENKEY-FAILURE get size fail");
                        return
                    }
                }

                match self.genkey(args[1], size) {
                    Ok(hash) => {
                        println!("GENKEY-SUCCESS {}", hash)
                    },
                    Err(_) => {
                        println!("GENKEY-FAILURE hash failed")
                    }
                }
            },

            "VERIFYKEYCONTENT" => {
                if args.len() < 3 {
                    verify_fail();
                    return;
                }

                let size;

                match file_size(args[1]) {
                    Ok(sz) => { size = sz },
                    Err(_) => {
                        verify_fail();
                        return
                    }
                }

                if let Ok(h) = self.genkey(args[2], size) {
                    if h == args[1] {
                        println!("VERIFYKEYCONTENT-SUCCESS");
                    }
                    else {
                        verify_fail();
                    }
                }
                else {
                    verify_fail();
                }
            },

            _ => protocol_error()
        }
    }

    fn main_loop(&self) -> io::Result<()> {
        let mut line = String::new();

        loop {
            line.clear();

            match io::stdin().read_line(&mut line) {
                // EOF
                Ok(0) => {
                    break
                },
                // command
                Ok(_) => self.line_process(&line),
                Err(_) => protocol_error()
            }
        }
        Ok(())
    }

}
