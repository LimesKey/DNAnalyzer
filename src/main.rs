mod parser;
use std::io;
use std::path::Path;

fn main() {
    let mut dna_file_path = String::new();

    print!("Enter file location: ");
    io::stdin().read_line(&mut dna_file_path).unwrap();
    let dna_file_path = Path::new(dna_file_path.trim()).canonicalize().unwrap();
    
    match parser::parseFasta(&dna_file_path.as_path()) {
        Ok(_) => {
            // Handle successful parsing here
        }
        Err(err) => {
            // Handle error case here
            panic!();
        }
    }
    
}

