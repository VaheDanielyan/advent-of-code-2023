// Learning rust by doing advent of code
// https://adventofcode.com/2023/day/12
// Vahe Danielyan, 2023

use std::fs;
use std::io::prelude::*;

fn solve(springs : &String, nums : &Vec<usize>) -> i32 {
    if springs.is_empty() {
        return if nums.is_empty() { 1 } else { 0 };
    }
    if nums.is_empty() {
        return if springs.contains('#') { 0 } else { 1 };
    }

    let mut result = 0;

    let first_char = springs.chars().next().unwrap_or_default();

    if first_char == '.' || first_char == '?' {
        result += solve(&springs[1..].to_string(), nums);
    }

    if first_char == '#' || first_char == '?' {
        let first_num : usize = nums[0];
        if first_num <= springs.len() && !springs[..first_num].contains('.')
        && (first_num == springs.len() || springs.chars().nth(first_num) != Some('#'))
        {
            if first_num + 1 < springs.len() {
                result += solve(&springs[(first_num as usize + 1)..].to_string(), &nums[1..].to_vec());
            }
            else {
                result += solve(&"".to_string(), &nums[1..].to_vec());
            }
        }
    }
    return result
}

fn main() {
    let mut file = fs::File::open("input.txt").unwrap();
    let mut contents : String = String::new();
    let _ = file.read_to_string(&mut contents);

    let mut numbers : Vec<Vec<usize>> = Vec::new();
    let mut springs : Vec<String> = Vec::new();

    let mut numbers_p2 : Vec<Vec<usize>> = Vec::new();
    let mut springs_p2 : Vec<String> = Vec::new();

    for line in contents.lines() {
        if let Some((spr, num)) = line.split_once(' ') {
            springs.push(spr.to_string());
            springs_p2.push(format!("{}?{}?{}?{}?{}", spr, spr, spr, spr, spr));
            let nums : Vec<usize> = num
                .split(',')
                .filter_map(|num| num.parse().ok())
                .collect();
            numbers.push(nums.clone());

            numbers_p2.push(std::iter::repeat(nums).take(5).flatten().collect());
        }
    }
    
    let mut answ_p1 = 0;
    for i in 0..numbers.len() {
        answ_p1 += solve(&springs[i], &numbers[i]);
    }
    println!("Answer P1: {}", answ_p1);

    let mut answ_p2 = 0;
    for i in 0..numbers_p2.len() {
        answ_p2 += solve(&springs_p2[i], &numbers_p2[i]);
    }
    println!("Answer P2: {}", answ_p2);
}
