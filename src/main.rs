use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: sapt <apt-arguments>");
        std::process::exit(1);
    }

    let status = Command::new("sudo")
        .arg("apt")
        .args(&args)
        .stdin(Stdio::inherit()) // let user type password, confirmations, etc.
        .stdout(Stdio::inherit()) // show apt output directly
        .stderr(Stdio::inherit()) // show error messages directly
        .status()
        .expect("failed to run sudo apt");

    std::process::exit(status.code().unwrap_or(1));
}
