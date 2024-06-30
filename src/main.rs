use clap::Parser;
use modules::{
    format::{cc_description, cc_type},
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
    let cc_description = cc_description(cc_type);
    let cc_message = format!("{}: {}", cc_type, cc_description);

    commit(&cc_message);
}
