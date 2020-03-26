use clap::{App, Arg};
use kprober::Symbols;

fn run_ui(filter: &str) -> Vec<String> {
    let symbols = Symbols::new(filter);
    kprober::run_ui(symbols)
}

fn run_kprobes(selected: Vec<String>) {
    kprober::run_kprobes(selected);
}

fn main() {
    // parse command line arguments
    let matches = App::new("kprober")
        .arg(
            Arg::with_name("filter")
                .short("f")
                .long("filter")
                .help("Filter symbols")
                .takes_value(true),
        )
        .get_matches();
    let filter = matches.value_of("filter").unwrap_or("");

    // run ui to get selected symbols
    let selected = run_ui(filter);

    // run kprobes on selected symbols
    run_kprobes(selected);
}
