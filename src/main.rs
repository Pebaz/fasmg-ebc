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

    let output = Command::new("fasmg")
        .args(args)
        .env("INCLUDE", "include")
        .output()
        .expect("failed to execute process");

    println!("{:?}", output);

    if !output.status.success()
    {
        println!("{}", String::from_utf8(output.stderr).unwrap());
    }
}

