// Learning rust by doing advent of code
// https://adventofcode.com/2023/day/5
// Vahe Danielyan, 2023

use std::fs;
use std::io::prelude::*;
use std::env;


fn compute_min(all_ranges: &Vec<Vec<(i64, i64, i64)>>, seeds : &Vec<i64>) -> i64 {
    let mut to_convert : i64;
    let mut min_location: i64 = i64::MAX;
    for i in 0..seeds.len() {
        to_convert = seeds[i] as i64;
        for range_list in all_ranges {
            for range in range_list {
                let src = range.1;
                let dst = range.0;
                let range = range.2;
                if to_convert >= src && to_convert <= (src + range - 1) {
                    to_convert += dst - src;
                    break;
                }
            }
        }
        println!("{}", to_convert);
        if to_convert <= min_location {
            min_location = to_convert;
        }
    }
    return min_location;
}

fn compute_min_part2(all_ranges: &Vec<Vec<(i64, i64, i64)>>, seeds : &Vec<i64>) -> i64 {
    let mut to_convert : i64;
    let mut min_location: i64 = i64::MAX;
    for chunk in seeds.chunks(2) {
        for i in 0..chunk[1] {
            to_convert = chunk[0] + i;
            //println!("-{}-", to_convert);
            for range_list in all_ranges {
                for range in range_list {
                    let src = range.1;
                    let dst = range.0;
                    let range = range.2;
                    if to_convert >= src && to_convert <= (src + range - 1) {
                        to_convert += dst - src;
                        break;
                    }
                }
                //print!("{} ", to_convert);
            }
            //println!("{}", to_convert);
            if to_convert <= min_location {
                min_location = to_convert;
            }
        }
    }
    return min_location;
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let mut file = fs::File::open(&args[1]).unwrap();

    let mut contents : String = String::new();
    let _ = file.read_to_string(&mut contents);

    let mut splits : Vec<&str> = contents.
        split("\n\n").
        filter(|line| !line.is_empty())
        .collect();
    
    let seeds : Vec<i64> = splits[0]
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .filter_map(|number| number.parse::<i64>().ok())
        .collect();
    

    
    let mut all_ranges : Vec<Vec<(i64, i64, i64)>> = Vec::new();

    splits.remove(0);
    for split in splits {
        let tuples : Vec<(i64, i64, i64)> = split
            .lines()
            .filter_map(|line| {
                let nums : Vec<i64> = line
                    .split_whitespace()
                    .filter_map(|num| num.parse().ok())
                    .collect();
                if nums.len() == 3 {
                    Some((nums[0], nums[1], nums[2]))
                }
                else {
                    None
                }
            })
            .collect();
        all_ranges.push(tuples);
    }

    //println!("Aswer to part1: {}", compute_min(&all_ranges, &seeds));
    
    println!("Aswer to part2: {}", compute_min_part2(&all_ranges, &seeds));

}
