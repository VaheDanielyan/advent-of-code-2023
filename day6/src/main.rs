// Learning rust by doing advent of code
// https://adventofcode.com/2023/day/6
// Vahe Danielyan, 2023

use std::fs;
use std::io::prelude::*;

fn get_win_range(t : f64, l : f64) -> (f64, f64) {
    let disc = ((t * t - 4f64 * l)).sqrt();
    let x1 = (t - disc) / 2f64;
    let x2 = (t + disc) / 2f64;
    return (x1, x2);
}

fn sol_part1(lines : &Vec<&str>) -> i64 {
    let times_line = lines[0];
    let distance_line = lines[1];

    let times : Vec<i64> = times_line
        .split(':')
        .collect::<Vec<&str>>()[1]
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();

    let distances : Vec<i64> = distance_line
        .split(':')
        .collect::<Vec<&str>>()[1]
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();

    for time in &times {
        print!("{} ", time); 
    }
    println!();
    for dist in &distances {
        print!("{} ", dist); 
    }
    println!();
    
    let mut answer_part1 : i64 = 1;
    for i in 0..times.len() {
        let (start, end) = get_win_range(times[i] as f64, distances[i] as f64);
        let int_start = std::cmp::max(start.ceil() as i64, start.floor() as i64 + 1);
        let int_end = std::cmp::min(end.floor() as i64, end.ceil() as i64 - 1);
        println!("{} - {} {} - {}", start, int_start, end, int_end);
        let win_numbers : Vec<i64> = (int_start as i64..=int_end).collect();
        answer_part1 *= win_numbers.len() as i64;
        println!(", length - {} ", win_numbers.len());
    }
    return answer_part1;
}

fn sol_part2(lines: &Vec<&str>) -> i64 {
    let time : i64 = lines[0].chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap();
    
    let dist : i64 = lines[1].chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap();

    let (start, end) = get_win_range(time as f64, dist as f64);
    let int_start = std::cmp::max(start.ceil() as i64, start.floor() as i64 + 1);
    let int_end = std::cmp::min(end.floor() as i64, end.ceil() as i64 - 1);
    println!("{} - {} {} - {}", start, int_start, end, int_end);
    let win_numbers : Vec<i64> = (int_start as i64..=int_end).collect();
    println!("{} ss", win_numbers.len());
    return win_numbers.len() as i64;
}

fn main() {
    let mut file = fs::File::open("input.txt").unwrap();

    let mut contents : String = String::new();
    let _ = file.read_to_string(&mut contents);
    
    let lines : Vec<&str> = contents
        .lines()
        .collect();
    
    println!("{} ", sol_part1(&lines));
    sol_part2(&lines);
}
