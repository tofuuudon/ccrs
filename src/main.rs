use clap::Parser;
use modules::{
    format::{cc_body, cc_breaking_change, cc_description, cc_scope, cc_type},
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
    buffer = cc_description(&buffer);
    buffer = cc_body(&buffer);

    commit(&buffer);
}
