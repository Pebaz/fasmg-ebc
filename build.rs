// FASMG not found on path, downloading
// FASMG failed to download: ERROR
// Please install FASMG manually by downloading from here and adding to your
// PATH

// use request;
use std::io;
use std::fs::File;
use std::process::Command;

const URL: &'static str = "http://flatassembler.net/fasmg.jg8x.zip";
const ZIP_FILE: &'static str = "fasmg.zip";

fn main()
{
    if let Err(_) = Command::new("fasmg").output()
    {
        println!("FASMG not found on path, downloading...");

        let mut body = ureq::get(URL)
            .set("Example-Header", "header value")
            .call()
            .unwrap()
            .into_reader();

        let mut zip_file = File::create(ZIP_FILE).expect(&format!(
            "Failed to open file {}. Permissions issue?",
            ZIP_FILE
        ));

        io::copy(&mut body, &mut zip_file).expect(&format!(
            "Failed to write file {}. Permissions issue?",
            ZIP_FILE
        ));
    }



    println!("cargo:rerun-if-changed=build.rs");
}
