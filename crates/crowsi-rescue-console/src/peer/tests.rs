use std::{
    fs,
    io::{BufRead, BufReader},
    os::unix::fs::PermissionsExt,
    process::{Child, Command, Stdio},
};

use sha2::{Digest, Sha256};

struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[test]
fn path_replacement_cannot_change_the_running_executable_digest() {
    let root = tempfile::tempdir().expect("temporary root");
    let executable = root.path().join("worker");
    // Build our own ELF so distribution-specific shell/interpreter wrappers
    // cannot be mistaken for the process's executable image.
    let source = root.path().join("worker.rs");
    fs::write(&source, "fn main() { use std::io::Write; println!(\"ready\"); std::io::stdout().flush().unwrap(); std::thread::sleep(std::time::Duration::from_secs(30)); }")
        .expect("native worker source");
    let compiler = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    assert!(
        Command::new(compiler)
            .arg("--edition=2024")
            .arg(&source)
            .arg("-o")
            .arg(&executable)
            .status()
            .expect("compile worker")
            .success()
    );
    let original = fs::read(&executable).expect("compiled worker");
    assert!(original.starts_with(b"\x7fELF"));
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o700)).expect("executable mode");
    let mut child = ChildGuard(
        Command::new(&executable)
            .stdout(Stdio::piped())
            .spawn()
            .expect("worker"),
    );
    let mut ready = String::new();
    BufReader::new(child.0.stdout.take().expect("worker stdout"))
        .read_line(&mut ready)
        .expect("worker readiness");
    assert_eq!(ready.trim(), "ready");

    fs::remove_file(&executable).expect("unlink executing path");
    fs::write(&executable, b"replacement image").expect("replace path");
    let digest = super::executable_digest(child.0.id()).expect("running inode digest");

    assert_eq!(
        digest,
        format!("sha256:{}", hex::encode(Sha256::digest(original)))
    );
}
