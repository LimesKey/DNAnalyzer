use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

use errors::*;

mod errors {
    use std::{error::Error, fmt};
    #[derive(Debug)]
    pub struct EmptyLine {
        pub line: u8
    }
    #[derive(Debug)]
    pub struct TextEncodingError {
        pub line: u8
    }

    #[derive(Debug)]
    pub enum Errors {
        EmptyLine(EmptyLine),
        TextEncodingError(TextEncodingError),
        IOError(std::io::Error),
    }

    impl Error for EmptyLine {}
    impl Error for TextEncodingError {}
    impl Error for Errors {}

    impl fmt::Display for EmptyLine {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Empty line at line number: {}", self.line)
        }
    }

    impl fmt::Display for TextEncodingError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Text encoding error at line number: {}", self.line)
        }
    }

    impl fmt::Display for Errors {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Errors::EmptyLine(err) => write!(f, "{}", err),
                Errors::TextEncodingError(err) => write!(f, "{}", err),
                Errors::IOError(err) => write!(f, "{}", err),
            }
        }
    }
}

pub fn parseFasta(file: &Path) -> Result<(), errors::Errors> {
    let file = match File::open(file) {
        Ok(file) => file,
        Err(error) => return Err(Errors::IOError(error)),
    };

    let mut file_content: Vec<String> = vec![];
    let reader = BufReader::new(file);
    let mut line_count: u8 = 0;
    let mut to_break = false;

    for raw_line in reader.lines() {
        line_count+=1;

        let mut line = match raw_line {
            Ok(raw_line) => raw_line,
            Err(error) => return Err(Errors::IOError(error)),
        };

        match line {
            _ if line.is_empty() => return Err(Errors::EmptyLine(EmptyLine{line: line_count})),
            _ if !(line.is_ascii()) => return Err(Errors::TextEncodingError(TextEncodingError{line: line_count})),
            _ => (),
        }

        if line.starts_with('>') { // File descriptor
            println!("\nReading DNA Description: {}", String::from(line[1..].trim()));
            continue;
        }

        if line.starts_with(';') { // Comment
            continue;
        }
        else if line.ends_with('*') {
            line = line.replace('*', "");
            to_break = true;
        }
        println!("Line {}: {}", line_count, line);

        file_content.push(line);

        if to_break {
            break;
        }

    }
    println!("sucess");
    Ok(())
}