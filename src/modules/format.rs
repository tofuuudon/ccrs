use std::io::{self, Write};

enum PromptType {
    Scope,
    BreakingChange,
    Description,
    Body,
}

pub enum PromptError {
    EmptyInput,
}

fn clear_lines(lines: u8) {
    print!("\x1B[{}A\x1B[2K", lines);
}

fn prompt(r#type: PromptType, prev: &str) -> String {
    match r#type {
        PromptType::Scope => {
            print!("{}(", prev)
        }
        PromptType::BreakingChange => {
            clear_lines(1);
            print!("{} <- has breaking change? y/N ", prev);
        }
        PromptType::Description => {
            clear_lines(1);
            print!("{}: ", prev);
        }
        PromptType::Body => {
            println!("");
        }
    }
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    buffer.trim().to_string()
}

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
        "rf" => "refactor",
        _ => "",
    }
}

pub fn cc_scope(prev: &str) -> String {
    let buffer = prompt(PromptType::Scope, prev);

    if buffer.trim().is_empty() {
        return prev.to_string();
    }

    format!("{}({})", prev, buffer.trim().to_string())
}

pub fn cc_breaking_change(prev: &str) -> String {
    let buffer = prompt(PromptType::BreakingChange, prev);

    match buffer.trim().to_lowercase().as_str() {
        "y" => format!("{}!", prev),
        _ => prev.to_string(),
    }
}

pub fn cc_description(prev: &str) -> Result<String, PromptError> {
    let buffer = prompt(PromptType::Description, prev);

    if buffer.is_empty() {
        return Err(PromptError::EmptyInput);
    }

    Ok(format!("{}: {}", prev, buffer.trim().to_string()))
}

pub fn cc_body(prev: &str) -> String {
    let buffer = prompt(PromptType::Body, prev);

    if buffer.trim().is_empty() {
        clear_lines(2);
        return prev.to_string();
    }

    format!("{}\n\n{}", prev, buffer.trim().to_string())
}

pub fn cc_confirm() -> bool {
    println!("\n--------------------");
    print!("Ready to commit? y/N ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    println!("");

    if buffer.trim().to_lowercase().as_str() == "y" {
        return true;
    };

    false
}
