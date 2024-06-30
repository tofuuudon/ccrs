pub mod git {
    use std::process::Command;

    pub fn commit(message: &str) {
        let output = Command::new("git")
            .arg("commit")
            .args(["--message", message])
            .output()
            .expect("failed");

        if output.status.success() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            eprintln!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }
}
