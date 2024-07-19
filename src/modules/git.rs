use regex::Regex;
use std::process::Command;

pub enum BumpType {
    Major,
    Minor,
    Patch,
    None,
}

pub fn commit(message: &str) {
    let output = Command::new("git")
        .arg("commit")
        .args(["--message", message])
        .output()
        .expect("failed");

    if output.status.success() {
        println!("\n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("\n{}", String::from_utf8_lossy(&output.stdout));
    }
}

pub fn get_latest_tag() -> String {
    let output = Command::new("git")
        .arg("describe")
        .args(["--tags", "--abbrev=0"])
        .output()
        .expect("failed");

    if !output.status.success() {
        eprintln!("\n{}", String::from_utf8_lossy(&output.stdout));
    }

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn get_version_numbers(tag: &str) -> (i8, i8, i8) {
    let re = Regex::new(r"v?(?<major>\d*)\.(?<minor>\d*)\.(?<patch>\d*)").unwrap();

    let caps = re.captures(tag).unwrap();
    let major = &caps["major"].parse::<i8>().unwrap();
    let minor = &caps["minor"].parse::<i8>().unwrap();
    let patch = &caps["patch"].parse::<i8>().unwrap();

    (*major, *minor, *patch)
}

pub fn get_bump_type(tag: &str) -> BumpType {
    let history = format!("{}..HEAD", tag);
    let output = Command::new("git")
        .arg("log")
        .args([
            &history,
            "--oneline",
            "--no-merges",
            "--no-decorate",
            "--format='%s'",
        ])
        .output()
        .expect("failed");

    if !output.status.success() {
        eprintln!("\n{}", String::from_utf8_lossy(&output.stdout));
    }

    let commits = String::from_utf8_lossy(&output.stdout).to_string();

    let major_re = Regex::new(r".*!:.*").unwrap();
    let minor_re = Regex::new(r"feat.*:.*").unwrap();
    let patch_re = Regex::new(r"fix.*:.*").unwrap();

    for line in commits.lines() {
        let bump_type = match line {
            _ if major_re.is_match(line) => BumpType::Major,
            _ if minor_re.is_match(line) => BumpType::Minor,
            _ if patch_re.is_match(line) => BumpType::Patch,
            _ => continue,
        };

        return bump_type;
    }

    BumpType::None
}

pub fn get_version_prefix(tag: &str) -> String {
    let re = Regex::new(r"(?<prefix>[a-zA-Z\W]*)\d*\.\d*\.\d*").unwrap();
    let caps = re.captures(tag).unwrap();

    caps["prefix"].to_string()
}

pub fn bump(tag: &str) {
    let output = Command::new("git")
        .arg("tag")
        .arg(tag)
        .output()
        .expect("failed");

    if !output.status.success() {
        println!("\n{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("\n{}", String::from_utf8_lossy(&output.stdout));
    }
}
