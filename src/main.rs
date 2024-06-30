use clap::Parser;

use format::format::cc_type;

mod format;

#[derive(Parser)]
struct Args {
    r#type: String,
}

fn main() {
    let args = Args::parse();

    let cc_type = cc_type(args.r#type.as_str());

    println!("{}", cc_type);
}
