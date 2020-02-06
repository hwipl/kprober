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
        for i in self.symbols.get() {
            select.add_item(format!("[ ] {}", i), i.to_string());
        }
        select.set_on_submit(Ui::on_submit);

        // main layer
        let select = select.with_name("select").scrollable().full_screen();
        self.siv.add_layer(
            Dialog::around(select)
                .title("Select Symbol")
                .button("Run/Quit", |s| s.quit()),
        );

        // start main loop
        self.siv.run();

        // return selected items
        let selected: &mut Vec<String> = self.siv.user_data().unwrap();
        selected.to_vec()
    }

    fn on_submit(s: &mut Cursive, name: &str) {
        // mark item as (not) selected in internal state
        let selected: &mut Vec<String> = s.user_data().unwrap();
        let mut removed = false;
        if selected.contains(&name.to_string()) {
            // remove existing item from selected vector
            selected.retain(|x| x != name);
            removed = true;
        } else {
            // add item to selected vector
            selected.push(String::from(name));
        }

        // mark item as (not) selected in view
        let mut display_text = format!("[*] {}", name);
        if removed {
            display_text = format!("[ ] {}", name);
        }
        let mut select = s.find_name::<SelectView<String>>("select").unwrap();
        let selected_id = select.selected_id().unwrap();
        select.insert_item(selected_id, display_text, name.to_string());
        select.remove_item(selected_id + 1);
    }
}

pub fn run_kprobes(names: Vec<String>) {
    // if no symbols are selected, stop here
    if names.len() == 0 {
        return;
    }

    // create krobe strings for bpftrace input
    println!("Start running bpftrace for:");
    let mut probes = String::new();
    for name in names {
        println!("    {}", name);
        let probe = format!("kprobe:{0} {{ printf(\"{0}\\n\"); }} ", name);
        probes = format!("{}{}", probes, probe);
    }
    println!("bpftrace output:\n");

    // run bpftrace
    let mut cmd = Command::new("bpftrace");
    let mut proc = cmd.arg("-e").arg(probes).spawn().unwrap();

    // set handler for ctrl-c
    ctrlc::set_handler(move || {
        println!("\nReceived CTRL-C. Stopping...");
    })
    .expect("Error setting Ctrl-C handler");

    // wait until bpftrace terminates
    proc.wait().unwrap();
    println!("Finished running bpftrace");
}
