use clap::Parser;
use modules::{
    format::{cc_body, cc_breaking_change, cc_description, cc_scope, cc_type, PromptError},
    git::commit,
};

mod modules;

#[derive(Parser)]
struct Args {
    r#type: String,
}

fn main() {
    println!("");

    let args = Args::parse();

    let cc_type = cc_type(args.r#type.as_str());
    let mut buffer = String::from(cc_type);

    buffer = cc_scope(&buffer);
    buffer = cc_breaking_change(&buffer);

    match cc_description(&buffer) {
        Ok(new_buffer) => buffer = new_buffer,
        Err(PromptError::EmptyInput) => {
            print!("\x1B[1A\x1B[2K[ERROR] Must provide a description");
            std::process::exit(1);
        }
    };

    buffer = cc_body(&buffer);

    commit(&buffer);
}
