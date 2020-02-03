use clap::{App, Arg};
use kprober::Symbols;
use kprober::Ui;

fn main() {
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
    let symbols = Symbols::new(filter);
    let mut ui = Ui::new(symbols);
    ui.run();
}
