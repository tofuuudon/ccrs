pub mod format {
    pub fn cc_type(type_arg: &str) -> &str {
        match type_arg {
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
        }
    }
}
