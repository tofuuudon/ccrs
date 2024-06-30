use std::io::{self, Write};

pub fn cc_type(type_arg: &str) -> &str {
    match type_arg {
        "fe" => "feat",
        "fi" => "fix",
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

pub fn cc_message(cc_type: &str) -> String {
    print!("{}: ", cc_type);
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("failed");

    buffer
}
