use clap::Parser;
use modules::{
    format::{cc_message, cc_type},
    git::commit,
};

mod modules;

#[derive(Parser)]
struct Args {
    r#type: String,
}

fn main() {
    let args = Args::parse();

    let cc_type = cc_type(args.r#type.as_str());
    let cc_message = cc_message(cc_type);
    let cc_commit = format!("{}: {}", cc_type, cc_message);

    commit(&cc_commit);
}
