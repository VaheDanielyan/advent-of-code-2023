// Learning rust by doing advent of code
// https://adventofcode.com/2023/day/9
// Vahe Danielyan, 2023

use std::{fs};
use std::io::prelude::*;

fn get_last_number(vec : Vec<i32>) -> i32 {
    if vec.iter().all(|&elem| elem == 0) {
        return 0;
    }
    let mut new_vec : Vec<i32> = Vec::new();
    for i in 0..vec.len() - 1 {
        new_vec.push(vec[i + 1] - vec[i]); 
    }
    return vec.last().unwrap() + get_last_number(new_vec);
}

fn get_first_number(vec : Vec<i32>) -> i32 {
    if vec.iter().all(|&elem| elem == 0) {
        return 0;
    }
    let mut new_vec : Vec<i32> = Vec::new();
    for i in 0..vec.len() - 1 {
        new_vec.push(vec[i + 1] - vec[i]); 
    }
    let ret = vec.first().unwrap() - get_first_number(new_vec);
    return ret;
}

fn main() {
    let mut file = fs::File::open("input.txt").unwrap();
    let mut contents : String = String::new();
    let _ = file.read_to_string(&mut contents);

    let vecs : Vec<Vec<i32>> = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    
    let mut answ = 0;
    let mut answ_p2 = 0;
    for vec in &vecs {
        answ += get_last_number(vec.clone());
        answ_p2 += get_first_number(vec.clone());
    }
    println!("{} {}", answ, answ_p2);
}
