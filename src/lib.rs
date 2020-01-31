use std::fs;

use cursive::traits::*;
use cursive::views::{Dialog, SelectView};
use cursive::Cursive;

pub struct Symbols {
    symbols: Vec<String>,
}

impl Symbols {
    fn from_file(filename: &str) -> String {
        fs::read_to_string(filename).unwrap()
    }

    pub fn new() -> Symbols {
        let mut symbols = Vec::new();
        let contents = Symbols::from_file("/proc/kallsyms");

        for line in contents.lines() {
            let l: Vec<&str> = line.split_whitespace().collect();

            if l[1] == "t" || l[1] == "T" {
                symbols.push(String::from(l[2]));
            }
        }

        Symbols { symbols }
    }

    pub fn print(&self) {
        for s in &self.symbols {
            println!("{}", s);
        }
    }

    fn get(&self) -> &Vec<String> {
        &self.symbols
    }
}

pub struct Ui {
    siv: Cursive,
    symbols: Symbols,
}

impl Ui {
    pub fn new(symbols: Symbols) -> Ui {
        // cursive root
        let siv = Cursive::default();

        Ui { siv, symbols }
    }

    pub fn run(&mut self) {
        // select view
        let mut select = SelectView::<String>::new();
        select.add_all_str(self.symbols.get());
        select.set_on_submit(Ui::on_submit);

        // main layer
        self.siv.add_layer(
            Dialog::around(select.scrollable().full_screen())
                .title("Select Symbol")
                .button("Quit", |s| s.quit()),
        );

        // start main loop
        self.siv.run()
    }

    fn on_submit(s: &mut Cursive, name: &str) {
        s.add_layer(
            Dialog::text(format!("Name: {}\n", name))
                .title(format!("{}", name))
                .button("Return", |s| {
                    s.pop_layer();
                }),
        );
    }
}
