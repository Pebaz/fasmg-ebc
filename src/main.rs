use std::process::Command;
use std::io::prelude::*;
use fasmg_ebc_rs::*;

fn main()
{
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    if args.len() < 2
    {
        return println!(
            "Usage:\n    fasmg-ebc-rs <SOURCE.asm> <DESTINATION.efi>"
        );
    }

    let temp_dir = std::path::Path::new(&std::env::temp_dir())
        .join("fasmg-ebc-rs");

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
        .args(args)
        .env("INCLUDE", temp_dir)
        .output()
    {
        Ok(output) => output,
        Err(_) =>
        {
            return println!(
                "Could not find fasmg on your path.\nInstall it from: {}",
                "http://flatassembler.net/fasmg.jg8x.zip"
            );
        }
    };

    if !output.status.success()
    {
        print!("{}", String::from_utf8(output.stderr).unwrap());
    }

    print!("{}", String::from_utf8(output.stdout).unwrap());
}
