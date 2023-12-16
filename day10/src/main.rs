// Learning rust by doing advent of code
// https://adventofcode.com/2023/day/10
// Vahe Danielyan, 2023

use std::{fs};
use std::io::prelude::*;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut file = fs::File::open("input.txt").unwrap();
    let mut contents : String = String::new();
    let _ = file.read_to_string(&mut contents);

    let mut grid : Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    
    let mut sx : usize = 0;
    let mut sy : usize = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == 'S' {
                sx = x;
                sy = y;
            }
            print!("{}", grid[x][y]);
        }
        println!();
    }
    let mut queue : VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited : HashSet<(usize, usize)> = HashSet::new(); 
    
    let mut orig_s : HashSet<char> = HashSet::from(['|', '-', 'J', 'L', '7', 'F']);

    queue.push_back((sx, sy));
    visited.insert((sx, sy));
    while !queue.is_empty() {
        let (cx, cy) = queue.pop_front().unwrap();
        let ch = grid[cx][cy];
        if cx > 0 && "S|JL".contains(ch) && "|7F".contains(grid[cx - 1][cy]) && !visited.contains(&(cx - 1, cy)) {
            visited.insert((cx - 1, cy)); 
            queue.push_back((cx - 1, cy));
            if ch == 'S' {
                let intersection_set: HashSet<char> = HashSet::from(['|', 'J', 'L']);
                orig_s = orig_s.intersection(&intersection_set).cloned().collect();
            }
        }
        if cx < grid.len() && "S|7F".contains(ch) && "|JL".contains(grid[cx + 1][cy]) && !visited.contains(&(cx + 1, cy)) {
            visited.insert((cx + 1, cy)); 
            queue.push_back((cx + 1, cy));
            if ch == 'S' {
                let intersection_set: HashSet<char> = HashSet::from(['|', '7', 'F']);
                orig_s = orig_s.intersection(&intersection_set).cloned().collect();
            }
        }
        if cy > 0 && "S-J7".contains(ch) && "-LF".contains(grid[cx][cy - 1]) && !visited.contains(&(cx, cy - 1)) {
            visited.insert((cx, cy - 1)); 
            queue.push_back((cx, cy - 1));
            if ch == 'S' {
                let intersection_set: HashSet<char> = HashSet::from(['-', 'J', '7']);
                orig_s = orig_s.intersection(&intersection_set).cloned().collect();
            }
        }
        if cy < grid[0].len() && "S-LF".contains(ch) && "-J7".contains(grid[cx][cy + 1]) && !visited.contains(&(cx, cy + 1)) {
            visited.insert((cx, cy + 1)); 
            queue.push_back((cx, cy + 1));
            if ch == 'S' {
                let intersection_set: HashSet<char> = HashSet::from(['-', 'L', 'F']);
                orig_s = orig_s.intersection(&intersection_set).cloned().collect();
            }
        }
    }
    println!("Part1: {}", (visited.len() as f32 / 2f32).ceil());

    println!();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                grid[i][j] = orig_s.iter().next().unwrap().clone();
            }
            else if !visited.contains(&(i, j)) {
                grid[i][j] = '.';
            }
            print!("{}", grid[i][j]);
        }
        println!();
    }

    println!();
    let mut outside : HashSet<(usize, usize)> = HashSet::new();
    for (i, row) in grid.iter().enumerate() {
        let mut inside = false;
        let mut up = false;
        for (j, &ch) in grid[i].iter().enumerate() {
            if ch == '|' {
                inside = !inside;
            }
            else if "LF".contains(ch) {
                up = ch == 'L';
            }
            else if "7J".contains(ch) {
                if up {
                    if ch != 'J' {
                        inside = !inside;
                    }
                }
                else if ch != '7' {
                    inside = !inside;
                }
                up = false;
            }
            else if ch == '.' {
            }
            if !inside {
                outside.insert((i, j));
                print!("{}", 'O');
            }
            else {
                print!("{}", grid[i][j]);
            }
        }
        println!();
    }
    println!("{}", (grid.len() * grid[0].len()) - outside.union(&visited).count());
}
