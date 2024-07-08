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
    let reader = BufReader::new(file);
    let mut line_count: u8 = 0;

    for raw_line in reader.lines() {
        line_count+=1;
        assert!(reader.buffer().is_empty());

        let line = match raw_line {
            Ok(raw_line) => raw_line,
            Err(error) => return Err(Errors::IOError(error)),
        };

        match line {
            _ if line.is_empty() => return Err(Errors::EmptyLine(EmptyLine{line: line_count})),
            _ if !(line.is_ascii()) => return Err(Errors::TextEncodingError(TextEncodingError{line: line_count})),
        }


    }

    Ok(())
}