use std::process::{Command, ExitStatus};

fn main()
{
    let output = Command::new(r"C:\users\samuelwilder\Downloads\fasmw17328\fasm")
        .args(["hello.asm", "hello.efi"])
        .output()
        .expect("failed to execute process");

    // println!("{:?}", output);

    if !output.status.success()
    {
        println!("{}", String::from_utf8(output.stderr).unwrap());
    }
}
