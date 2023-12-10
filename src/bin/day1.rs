use std::{error::Error, io::BufRead};

fn find_str_or_digit(haystack: &str, needle: Option<&str>, first: bool) -> Option<usize> {
    match (first, needle) {
        (true, Some(needle)) => haystack.find(needle),
        (false, Some(needle)) => haystack.rfind(needle),
        (true, None) => haystack.find(|c: char| c.is_ascii_digit()),
        (false, None) => haystack.rfind(|c: char| c.is_ascii_digit()),
    }
}

fn find_first_or_last(line: &str, first: bool, only_digits: bool) -> i32 {
    let mut index = find_str_or_digit(line, None, first).expect("a digit");
    let mut digit = line.as_bytes()[index] as i32 - '0' as i32;
    if only_digits {
        return digit;
    }
    for (text, num) in [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ] {
        if let Some(i) = find_str_or_digit(line, Some(text), first) {
            if (first && i < index) || (!first && i > index) {
                index = i;
                digit = num;
            }
        }
    }
    return digit;
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "data/day1.txt".into());
    let first_part = match std::env::args().nth(2).as_deref() {
        None => true,
        Some("1" | "A" | "a") => true,
        Some("2" | "B" | "b") => false,
        _ => Err("Unknown part")?,
    };
    let lines = std::io::BufReader::new(std::fs::File::open(&filename)?).lines();
    let mut num = 0;
    for line in lines {
        let line = line?;
        let first_num = find_first_or_last(&line, true, first_part);
        let last_num = find_first_or_last(&line, false, first_part);
        // println!("{first_num} {last_num}");
        num += first_num * 10 + last_num;
    }
    println!("{num}");
    return Ok(());
}
