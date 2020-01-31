use kprober::Symbols;
use kprober::Ui;

fn main() {
    let symbols = Symbols::new();
    let mut ui = Ui::new(symbols);
    ui.run();
}
