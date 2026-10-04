use std::io::{self, Write, Error};
use std::fs::File;

pub fn get_full_name(first: &str, last: &str) -> String {
    String::from(first) + " " + last
}

pub fn generate_big_file(file_name: &str, size: u64) -> io::Result<()> {

    let pline = "0123456789012345678901234567890123456789".to_string();
    let mut file = File::create(file_name)?;
    let count_lines = size / 40u64;

    for i in 0 .. count_lines {
        writeln!(&mut file, "{} {}", pline, i)?;
    }
    Ok(())

}