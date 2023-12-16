// Learning rust by doing advent of code
// https://adventofcode.com/2023/day/11
// Vahe Danielyan, 2023

use std::io::prelude::*;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;
use std::{fs};

fn main() {
    let mut file = fs::File::open("input.txt").unwrap();
    let mut contents : String = String::new();
    let _ = file.read_to_string(&mut contents);

    let mut grid : Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    for row in &grid {
        for &ch in row {
            print!("{}", ch);
        }
        println!();
    }
    println!();

    let empty_rows : Vec<usize> = grid.iter()
        .enumerate()
        .filter(|&(ri, row)| {
            row.iter().all(|&ch| ch == '.')
        })
        .map(|(ri, row)| ri)
        .collect();

    for row_index in &empty_rows {
        print!("{} ", row_index);
    }
    println!();

    let cols = grid[0].len();
    let empty_cols: Vec<usize> = (0..cols)
        .filter(|&c| grid.iter().all(|row| row[c] == '.'))
        .collect();

    for col_index in &empty_cols {
        print!("{} ", col_index);
    }
    println!();

    let points: Vec<(usize, usize)> = grid.iter()
    .enumerate()
    .flat_map(|(r, row)| {
        row.iter()
            .enumerate()
            .filter(|&(_c, &ch)| ch == '#')
            .map(move |(c, _ch)| (r, c))
            .collect::<Vec<(usize, usize)>>()
    })
    .collect();

    for point in &points {
        print!("{} {} ,", point.0, point.1);
    }
    println!();

    let mut answ_p1 = 0;
    let mut answ_p2 : i64 = 0;
    let expansion_rate_p1 = 2;
    let expansion_rate_p2 = 1000000;
    for (i, &(x1, y1)) in points.iter().enumerate() {
        for &(x2, y2) in points.iter().take(i) {
            for r in (std::cmp::min(x1, x2)..std::cmp::max(x1,x2)).step_by(1) {
                if empty_rows.contains(&r) {
                    answ_p1 += expansion_rate_p1;
                    answ_p2 += expansion_rate_p2;
                } else {
                    answ_p1 += 1;
                    answ_p2 += 1;
                }
            }
            for c in (std::cmp::min(y1, y2)..std::cmp::max(y1,y2)).step_by(1) {
                if empty_cols.contains(&c) {
                    answ_p1 += expansion_rate_p1;
                    answ_p2 += expansion_rate_p2;
                } else {
                    answ_p1 += 1;
                    answ_p2 += 1;
                }
            }
        }
    }

    println!("P1: {}", answ_p1);
    println!("P2: {}", answ_p2);

}
