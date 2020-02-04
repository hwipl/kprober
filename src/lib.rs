use std::fs;
use std::process::Command;

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

    pub fn new(filter: &str) -> Symbols {
        let mut symbols = Vec::new();
        let contents = Symbols::from_file("/proc/kallsyms");

        for line in contents.lines() {
            if !line.contains(filter) {
                continue;
            }
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
        let mut siv = Cursive::default();

        // empty vector of selected items as user data
        let selected: Vec<String> = Vec::new();
        siv.set_user_data(selected);

        Ui { siv, symbols }
    }

    pub fn run(&mut self) -> Vec<String> {
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
        self.siv.run();

        // return selected items
        let selected: &mut Vec<String> = self.siv.user_data().unwrap();
        selected.to_vec()
    }

    fn on_submit(s: &mut Cursive, name: &str) {
        let cmd = String::from(name);
        s.add_layer(
            Dialog::text(format!("Name: {}\n", name))
                .title(format!("{}", name))
                .button("Return", |s| {
                    s.pop_layer();
                })
                .button("Probe", move |s| {
                    let selected: &mut Vec<String> = s.user_data().unwrap();
                    selected.push(String::from(&cmd));
                    s.quit();
                    s.clear();
                }),
        );
    }
}

pub fn run_commands(names: Vec<String>) {
    for name in names {
        println!("Start running command {}", name);
        let probe = format!("kprobe:{0} {{ printf(\"{0}\\n\"); }}", name);
        let mut cmd = Command::new("bpftrace");
        let mut proc = cmd.arg("-e").arg(probe).spawn().unwrap();
        proc.wait().unwrap();
        println!("Finished running command {}", name);
    }
}
