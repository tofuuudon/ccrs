use std::env;

use modules::{
    format::{
        cc_body, cc_breaking_change, cc_confirm, cc_description, cc_footer, cc_scope, cc_type,
        PromptError,
    },
    git::{
        bump, commit, get_bump_type, get_latest_tag, get_version_numbers, get_version_prefix,
        BumpType,
    },
};

mod modules;

fn run_commit(command: &str) {
    let cc_type = cc_type(command);
    let mut buffer = String::from(cc_type);

    buffer = cc_scope(&buffer);

    let (breaking_change, has_breaking_change) = cc_breaking_change(&buffer);
    buffer = breaking_change;

    match cc_description(&buffer) {
        Ok(new_buffer) => buffer = new_buffer,
        Err(PromptError::EmptyInput) => {
            print!("\x1B[1A\x1B[2K[ERROR] Must provide a description");
            std::process::exit(1);
        }
    };

    buffer = cc_body(&buffer);

    buffer = cc_footer(&buffer, has_breaking_change);

    cc_confirm();
    commit(&buffer);
}

fn run_bump() {
    let tag = get_latest_tag();
    let (major, minor, patch) = get_version_numbers(&tag);

    let new_version = match get_bump_type(&tag) {
        BumpType::Major => {
            println!("Bumping major ...");
            format!("{}.{}.{}", major + 1, 0, 0)
        }
        BumpType::Minor => {
            println!("Bumping minor ...");
            format!("{}.{}.{}", major, minor + 1, 0)
        }
        BumpType::Patch => {
            println!("Bumping patch ...");
            format!("{}.{}.{}", major, minor, patch + 1)
        }
        BumpType::None => {
            println!("[ERROR] No changes, skipping bump\n",);
            std::process::exit(1);
        }
    };
    let prefix = get_version_prefix(&tag);
    let new_tag = format!("{}{}", prefix, new_version);

    println!("\n{} -> {}", tag, new_tag);

    cc_confirm();
    bump(&new_tag);
}

fn main() {
    println!("\n------- ccrs -------\n");

    let command = match env::args().nth(1) {
        Some(v) => v,
        None => {
            println!("[ERROR] Invalid command\n",);
            std::process::exit(1);
        }
    };

    match command.as_str() {
        "bump" => run_bump(),
        _ => run_commit(&command),
    }
}
