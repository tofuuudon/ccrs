use std::env;

use modules::{
    format::{
        cc_body, cc_breaking_change, cc_confirm, cc_description, cc_footer, cc_scope, cc_type,
        PromptError,
    },
    git::commit,
};

mod modules;

fn main() {
    println!("\n------- ccrs -------\n");

    let command = env::args().nth(1).unwrap();

    let cc_type = cc_type(&command);
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
    buffer = cc_footer(&buffer);

    cc_confirm();

    commit(&buffer);
}
