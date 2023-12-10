// Learning rust by doing advent of code
// https://adventofcode.com/2023/day/4
// Vahe Danielyan, 2023

use std::{fs, vec};
use std::io::{prelude::*, self};

fn main() {
    let file = fs::File::open("input.txt").unwrap();

    let lines : Vec<String> = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok())
        .collect();
    
    let mut answ_sum : i32 = 0;
    let mut card_count = vec![1; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        let game_data : Vec<&str> = line.split(':')
            .collect();

        let parts : Vec<&str> = game_data[1].split('|').collect();
        let lucky_numbers : Vec<i32> = parts[0]
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let ticket_numbers : Vec<i32> = parts[1]
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let mut count : u32 = 0;
        for num in &ticket_numbers {
            if lucky_numbers.contains(&num) {
                count += 1;
            }
        }
        if count != 0 {
            answ_sum += 2_i32.pow(count - 1);
        }
        for _ in 0..card_count[i] {
            for j in 0..count {
                card_count[i + 1 + j as usize] += 1;  
            }
        }
    }
    println!("{}", answ_sum);
    let number_of_cards : u32 = card_count.iter().sum();
    println!("{}", number_of_cards);
}
