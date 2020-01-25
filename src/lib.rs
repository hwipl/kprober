use std::fs;

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

        Symbols{ symbols }
    }

    pub fn print(&self) {
        for s in &self.symbols {
            println!("{}", s);
        }
    }
}
