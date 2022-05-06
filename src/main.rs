use clap::{Arg, Command};
use kprober::Symbols;

fn run_ui(source: &str, filter: &str) -> Vec<String> {
    let symbols = Symbols::new(source, filter);
    kprober::run_ui(symbols)
}

fn run_kprobes(selected: Vec<String>) {
    kprober::run_kprobes(selected);
}

fn main() {
    // parse command line arguments
    let matches = Command::new("kprober")
        .arg(
            Arg::new("filter")
                .short('f')
                .long("filter")
                .help("Filter symbols")
                .takes_value(true),
        )
        .arg(
            Arg::new("symbol-source")
                .short('s')
                .help("Symbol source")
                .takes_value(true)
                .possible_value("kallsyms")
                .possible_value("bpftrace"),
        )
        .get_matches();
    let filter = matches.value_of("filter").unwrap_or("");
    let source = matches.value_of("symbol-source").unwrap_or("kallsyms");

    // run ui to get selected symbols
    let selected = run_ui(source, filter);

    // run kprobes on selected symbols
    run_kprobes(selected);
}
