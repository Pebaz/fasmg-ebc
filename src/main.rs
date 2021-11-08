use std::process::Command;

fn main()
{
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    if args.len() < 2
    {
        return println!(
            "Usage:\n    fasmg-ebc-rs <SOURCE.asm> <DESTINATION.efi>"
        );
    }

    let output = match Command::new("fasmg")
        .arg("-n")
        .args(args)
        .env("INCLUDE", "include")
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
