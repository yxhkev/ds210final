use std::env;
use std::fs; 
use std::io; 
use std::error::Error;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    
    let mut csvreader = CsvReader::new();

    // Read the file
    if let Err(e) = csvreader.read_csv(Snap) {
        println!("Error: {}", e);
        process::exit(1);
    }

    // Print out the contents of the CSV
    println!("Contents of {}:", filename);
    for line in csvreader.lines {
        for cell in line {
            print!("{}\t", cell);
        }
        println!("");
    }
}

struct CsvReader {
    lines: Vec<Vec<String>>,
}

impl CsvReader {
    fn new() -> CsvReader {
        CsvReader { lines: Vec::new() }
    }

    fn read_csv(&mut self, filename: &str) -> Result<(), io::Error> {
        let content = fs::read_to_string(filename)?;
        let lines = content.lines();

        for line in lines {
            let cells = line.split(",").map(|cell| cell.to_string()).collect();
            self.lines.push(cells);
        }
        Ok(())
    }
}