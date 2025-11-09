use std::fs;
use std::io;

pub fn save_csv(filename: &str, content: &str) -> io::Result<()> {
    fs::write(filename, format!("\u{FEFF}{}", content))?;

    Ok(())
}
