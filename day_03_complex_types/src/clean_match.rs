enum ErrorTypes {
    IoError(std::io::Error),
    FormatError(std::fmt::Error),
}

struct ErrorWrapper {
    source: ErrorTypes,
    message: String,
}

impl From<std::io::Error> for ErrorWrapper {
    fn from(source: std::io::Error) -> Self {
        Self {
            source: ErrorTypes::IoError(source),
            message: "I/O error".into(),
        }
    }
}

fn write_to_file() -> Result<(), ErrorWrapper> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::create("example.txt")?;
    file.write_all(b"Hello, World!")?;
    Ok(())
}

pub fn main() {
    match write_to_file() {
        Ok(()) => println!("Write succeeded"),
        Err(err) => {
            println!("Error writing to file: {}", err.message);
        }
    }
}
