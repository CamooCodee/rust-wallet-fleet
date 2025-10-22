use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn has_cmd(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}

fn spawn(cmd: &str, args: &[&str], name: &str) -> std::process::Child {
    let mut child = Command::new(cmd)
        .args(args)
        .env("RUST_LOG", "info")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect(name);
    child
}

fn main() {
    let mut be = if has_cmd("cargo-watch") {
        spawn(
            "cargo",
            &["watch", "-q", "-c", "-w", "backend", "-x", "run -p backend"],
            "backend",
        )
    } else {
        spawn("cargo", &["run", "-p", "backend"], "backend")
    };
    thread::sleep(Duration::from_millis(500));
    let mut fe = spawn(
        "npm",
        &["run", "dev", "--prefix", "frontend/wallet-fleet"],
        "wallet-fleet",
    );
    let _ = be.wait();
    let _ = fe.wait();
}
