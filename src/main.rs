use clap::Parser;

#[derive(Parser)]
struct Args {
    r#type: String,
}

fn main() {
    let args = Args::parse();

    let cc_type = match args.r#type.as_str() {
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
    };

    println!("{}", cc_type);
}
