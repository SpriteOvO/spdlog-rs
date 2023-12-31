use spdlog::formatter::{pattern, Pattern};

fn custom_pat_creator() -> impl Pattern {
    unimplemented!()
}

fn pattern() {
    pattern!("{logger} {$custom_pat}", {$custom_pat} => custom_pat_creator, {$} => custom_pat_creator);
    pattern!("{logger} {$custom_pat}", {$custom_pat} => custom_pat_creator, {$custom-pat2} => custom_pat_creator);
    pattern!("{logger} {$custom_pat}", {$custom_pat} => custom_pat_creator, {$2custom_pat} => custom_pat_creator);
}

fn main() {}
