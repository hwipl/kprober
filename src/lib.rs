use std::fs;

pub struct Symbols {
    symbols: String,
}

impl Symbols {
    pub fn from_file(filename: &str) -> Symbols {
        let symbols = fs::read_to_string(filename).unwrap();
        Symbols { symbols }
    }

    pub fn new() -> Symbols {
        Symbols::from_file("/proc/kallsyms")
    }

    pub fn print(&self) {
        for line in self.symbols.lines() {
            let l: Vec<&str> = line.split_whitespace().collect();
            if l[1] == "t" || l[1] == "T" {
                println!("{}", l[2]);
            }
        }
    }
}
