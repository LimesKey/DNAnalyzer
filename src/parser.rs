use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use errors::*;
use crate::parser::errors::Errors;

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

    impl Error for EmptyLine {}
    impl Error for TextEncodingError {}

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

    pub enum Errors {
        EmptyLine(EmptyLine),
        IOError(std::io::Error),
        TextEncodingError(TextEncodingError),
    }
}

fn parseFasta(file: File) -> Result<(), errors::Errors>{
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

        if line.starts_with(">") { // File descriptor
            println!("Reading DNA: {}", String::from(line[1..].trim()));
            continue;
        }

        if line.starts_with(";") { // Comment
            continue;
        }
        else if line.ends_with("*") {
            line = line.replace("*", "");
            to_break = true;
        }

        file_content.push(line);

        if to_break {
            break;
        }

    }
    Ok(())
}