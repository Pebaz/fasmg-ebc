use std::process::Command;
use std::io::prelude::*;

pub const EBC: &[u8] = include_bytes!("include/ebc.inc");
pub const EFI: &[u8] = include_bytes!("include/efi.inc");
pub const FORMAT: &[u8] = include_bytes!("include/format.inc");
pub const PE: &[u8] = include_bytes!("include/pe.inc");
pub const UTF8: &[u8] = include_bytes!("include/utf8.inc");
pub const STUB: &[u8] = include_bytes!("include/stub.com");

/// Provided so that other crates can compile EFI Bytecode
pub fn assemble_ebc(input_filename: &str, output_filename: &str)
{
    let temp_dir = std::path::Path::new(&std::env::temp_dir()).join(
        std::env::var("CARGO_PKG_NAME").unwrap()
    );

    std::fs::create_dir(&temp_dir).unwrap();

    let files = [
        ("ebc.inc", EBC),
        ("efi.inc", EFI),
        ("format.inc", FORMAT),
        ("pe.inc", PE),
        ("utf8.inc", UTF8),
        ("stub.com", STUB)
    ];

    for (filename, file_content) in files.into_iter()
    {
        let full_path = temp_dir.join(filename);
        let mut file = std::fs::File::create(full_path).unwrap();
        file.write_all(file_content).unwrap();
    }

    let output = match Command::new("fasmg")
        .arg("-n")
        .args([input_filename, output_filename])
        .env("INCLUDE", temp_dir)
        .output()
    {
        Ok(output) => output,
        Err(_) =>
        {
            panic!(
                "Could not find fasmg on your path.\nInstall it from: {}",
                "http://flatassembler.net/fasmg.jg8x.zip"
            );
        }
    };

    if !output.status.success()
    {
        panic!("{}", String::from_utf8(output.stderr).unwrap());
    }
}
