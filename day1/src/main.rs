// Learning rust by doing advent of code
// https://adventofcode.com/2023/day/1
// Vahe Danielyan, 2023

use std::io::{prelude::*, self};
use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new("a.txt");
    
    let file = match File::open(&path) {
        Err(why) => panic!("AAA {}", why),
        Ok(file) => file,
    };

    let lines : io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();
    let mut sum : u32 = 0;
    for line_result in lines {
        let line = line_result.unwrap();
        let mut first_found : bool = false;
        let mut first_digit : char = Default::default();
        let mut last_digit : char = Default::default();
        for ch in line.chars() {
            if ch >= '0' && ch <= '9' {
                if first_found == false {
                    first_digit = ch;
                    first_found = true;
                }
                last_digit = ch;
            }
        }
        println!("{} {}", first_digit, last_digit);
        sum += first_digit.to_digit(10).unwrap() * 10 + last_digit.to_digit(10).unwrap();
    }
    println!("{}", sum);
}
