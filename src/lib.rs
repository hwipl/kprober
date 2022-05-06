use std::fs;
use std::process::Command;

use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView, TextView};
use cursive::Cursive;

pub struct Symbols {
    symbols: Vec<String>,
}

impl Symbols {
    fn from_file(filename: &str) -> String {
        fs::read_to_string(filename).unwrap()
    }

    fn from_bpftrace() -> String {
        // run bpftrace and capture output
        let output = Command::new("bpftrace")
            .arg("-l")
            .arg("kprobe:*")
            .output()
            .expect("failed to execute \"bpftrace -l\"");

        return String::from_utf8(output.stdout).unwrap();
    }

    fn new_from_file(filter: &str) -> Vec<String> {
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
        return symbols;
    }

    fn new_from_bpftrace(filter: &str) -> Vec<String> {
        let mut symbols = Vec::new();
        let contents = Symbols::from_bpftrace();

        for line in contents.lines() {
            if !line.contains(filter) {
                continue;
            }
            let line = String::from(line.strip_prefix("kprobe:").unwrap());
            symbols.push(line);
        }
        return symbols;
    }

    pub fn new(source: &str, filter: &str) -> Symbols {
        let mut symbols: Vec<String>;

        if source == "bpftrace" {
            symbols = Symbols::new_from_bpftrace(filter);
        } else {
            symbols = Symbols::new_from_file(filter);
        };

        symbols.sort();
        Symbols { symbols }
    }

    pub fn print(&self) {
        for s in &self.symbols {
            println!("{}", s);
        }
    }

    fn filter(&self, filter: &str) -> Vec<String> {
        let mut symbols: Vec<String> = Vec::new();
        for s in &self.symbols {
            if !s.contains(filter) {
                continue;
            }
            symbols.push(s.to_string());
        }
        return symbols;
    }

    fn get(&self) -> &Vec<String> {
        &self.symbols
    }
}

struct UiState {
    symbols: Symbols,
    selected: Vec<String>,
}

pub fn run_ui(symbols: Symbols) -> Vec<String> {
    // cursive root
    let mut siv = cursive::default();

    // select view
    let mut select = SelectView::<String>::new();
    for i in symbols.get() {
        select.add_item(format!("[ ] {}", i), i.to_string());
    }
    select.set_on_submit(ui_on_submit);

    // bottom layout
    let mut edit = EditView::new();
    edit.set_on_submit(ui_on_edit_submit);
    let bottom_layout = LinearLayout::horizontal()
        .child(TextView::new("Filter:"))
        .child(edit.fixed_width(60))
        .child(DummyView.full_width())
        .child(Button::new("Run/Quit", |s| s.quit()));

    // main layout
    let select = select.with_name("select").scrollable().full_screen();
    let layout = LinearLayout::vertical()
        .child(select)
        .child(DummyView)
        .child(bottom_layout);
    siv.add_layer(Dialog::around(layout).title("Select Symbol"));

    // create ui state as user data
    let state = UiState {
        symbols,
        selected: Vec::new(),
    };
    siv.set_user_data(state);

    // start main loop
    siv.run();

    // return selected items
    let state: &mut UiState = siv.user_data().unwrap();
    let selected: &mut Vec<String> = &mut state.selected;
    selected.to_vec()
}

fn ui_on_submit(s: &mut Cursive, name: &str) {
    // mark item as (not) selected in internal state
    let state: &mut UiState = s.user_data().unwrap();
    let selected: &mut Vec<String> = &mut state.selected;
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

fn ui_on_edit_submit(s: &mut Cursive, name: &str) {
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();
    let state: &mut UiState = s.user_data().unwrap();
    select.clear();
    for i in state.symbols.filter(name) {
        if state.selected.contains(&i.to_string()) {
            select.add_item(format!("[*] {}", i), i.to_string());
        } else {
            select.add_item(format!("[ ] {}", i), i.to_string());
        }
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
