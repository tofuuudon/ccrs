use std::io::{self, Write};

pub fn cc_type(type_arg: &str) -> &str {
    match type_arg {
        "ft" => "feat",
        "fx" => "fix",
        "do" => "docs",
        "st" => "style",
        "p" => "perf",
        "t" => "test",
        "b" => "build",
        "ci" => "ci",
        "ch" => "chore",
        _ => "",
    }
}

pub fn cc_scope(prev: &str) -> String {
    print!("{}(", prev);
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    if buffer.trim().is_empty() {
        return prev.to_string();
    }

    format!("{}({})", prev, buffer.trim().to_string())
}

pub fn cc_breaking_change(prev: &str) -> String {
    print!("\x1B[1A\x1B[2K");
    print!("{} <- has breaking change? [y/N] ", prev);
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    match buffer.to_lowercase().as_str().trim() {
        "y" => format!("{}!", prev),
        _ => prev.to_string(),
    }
}

pub fn cc_description(prev: &str) -> String {
    print!("\x1B[1A\x1B[2K");
    print!("{}: ", prev);
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    format!("{}: {}", prev, buffer.trim().to_string())
}

pub fn cc_body(prev: &str) -> String {
    println!("");

    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    if buffer.trim().is_empty() {
        prev.to_string();
    }

    format!("{}\n\n{}", prev, buffer.trim().to_string())
}
