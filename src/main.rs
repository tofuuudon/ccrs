use clap::Parser;

use format::format::cc_message;
use format::format::cc_type;

mod format;

#[derive(Parser)]
struct Args {
    r#type: String,
}

fn main() {
    let args = Args::parse();

    let cc_type = cc_type(args.r#type.as_str());
    let cc_message = cc_message(cc_type);

    println!("[Done] {}: {}", cc_type, cc_message);
}
