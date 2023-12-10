use std::{error::Error, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "data/day1.txt".into());
    let lines = std::io::BufReader::new(std::fs::File::open(&filename)?).lines();
    let mut num = 0;
    for line in lines {
        let line = line?;
        let first_num = line.as_bytes()[(&line)
            .find(|c: char| c.is_ascii_digit())
            .ok_or("No digit found")?];
        let last_num = line.as_bytes()[(&line).rfind(|c: char| c.is_ascii_digit()).unwrap()];
        const ZERO: i32 = '0' as i32;
        num += (first_num as i32 - ZERO) * 10 + last_num as i32 - ZERO;
    }
    println!("{num}");
    return Ok(());
}
