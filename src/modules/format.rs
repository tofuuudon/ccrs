use std::io::{self, Write};

enum PromptType {
    Scope,
    BreakingChange,
    Description,
    Body,
    Footer,
    FooterBreakingChange,
}

pub enum PromptError {
    EmptyInput,
}

fn clear_lines(n: u8) {
    for _i in 0..n {
        print!("\x1B[1A\x1B[2K");
    }
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
            print!("[body] ");
        }
        PromptType::Footer => {
            println!("");
            print!("[footer] ");
        }
        PromptType::FooterBreakingChange => {
            println!("");
            print!("BREAKING CHANGE: ");
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

pub fn cc_breaking_change(prev: &str) -> (String, bool) {
    let buffer = prompt(PromptType::BreakingChange, prev);

    match buffer.trim().to_lowercase().as_str() {
        "y" => (format!("{}!", prev), true),
        _ => (prev.to_string(), false),
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

    let body = buffer.trim().to_string();
    clear_lines(1);
    print!("{}\n", body);

    format!("{}\n\n{}", prev, body)
}

pub fn cc_footer(prev: &str, has_breaking_change: bool) -> String {
    let prompt_type = if has_breaking_change {
        PromptType::FooterBreakingChange
    } else {
        PromptType::Footer
    };
    let buffer = prompt(prompt_type, prev);

    if buffer.trim().is_empty() {
        clear_lines(2);
        return prev.to_string();
    }

    let footer = buffer.trim().to_string();
    clear_lines(1);

    if has_breaking_change {
        print!("BREAKING CHANGE: {}\n", footer);
        return format!("{}\n\nBREAKING CHANGE: {}", prev, footer);
    }

    print!("{}\n", footer);
    format!("{}\n\n{}", prev, footer)
}

pub fn cc_confirm() {
    println!("\n--------------------");
    print!("Ready to commit? [Enter] ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    println!("");
}
