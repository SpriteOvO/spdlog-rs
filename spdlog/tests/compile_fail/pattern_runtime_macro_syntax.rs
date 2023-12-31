use spdlog::formatter::{runtime_pattern, Pattern};

fn custom_pat_creator() -> impl Pattern {
    unimplemented!()
}

fn runtime_pattern() {
    runtime_pattern!("{logger} {$custom_pat}", {$custom_pat} => custom_pat_creator, {$} => custom_pat_creator);
    runtime_pattern!("{logger} {$custom_pat}", {$custom_pat} => custom_pat_creator, {$custom-pat2} => custom_pat_creator);
    runtime_pattern!("{logger} {$custom_pat}", {$custom_pat} => custom_pat_creator, {$2custom_pat} => custom_pat_creator);
}

fn main() {}
