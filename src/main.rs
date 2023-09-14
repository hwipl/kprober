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
                .default_value(""),
        )
        .arg(
            Arg::new("symbol-source")
                .short('s')
                .help("Symbol source")
                .value_parser(["kallsyms", "bpftrace"])
                .default_value("kallsyms"),
        )
        .get_matches();
    let filter = matches.get_one::<String>("filter").unwrap();
    let source = matches.get_one::<String>("symbol-source").unwrap();

    // run ui to get selected symbols
    let selected = run_ui(source, filter);

    // run kprobes on selected symbols
    run_kprobes(selected);
}
