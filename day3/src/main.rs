// Learning rust by doing advent of code
// https://adventofcode.com/2023/day/3
// Vahe Danielyan, 2023

use std::fs;
use std::io::Read;
use std::collections::HashMap;

fn part1_solution(matrix: &Vec<Vec<char>>) -> u32 {
    let mut mark_matrix = vec![vec!['.'; matrix.len()]; matrix[0].len()];
    for (i, line) in matrix.iter().enumerate() {
        for (j, &ch) in line.iter().enumerate() {
            if !ch.is_digit(10) && ch != '.' {
                mark_matrix[i][j] = 'x';
                if i > 0 {
                    mark_matrix[i - 1][j] = 'x'; // Up
                    if j > 0 { mark_matrix[i - 1][j - 1] = 'x'; } // Diagonal Up-Left
                    if j < matrix[0].len() - 1 { mark_matrix[i - 1][j + 1] = 'x'; } // Diagonal Up-Right
                }
                if j > 0 { mark_matrix[i][j - 1] = 'x'; } // Left
                if i < matrix.len() - 1 {
                    mark_matrix[i + 1][j] = 'x'; // Down
                    if j > 0 { mark_matrix[i + 1][j - 1] = 'x'; } // Diagonal Down-Left
                    if j < matrix[0].len() - 1 { mark_matrix[i + 1][j + 1] = 'x'; } // Diagonal Down-Right
                }
                if j < matrix[0].len() - 1 { mark_matrix[i][j + 1] = 'x'; } // Right
            }
        }
    }
    for row in &mark_matrix {
        for &ch in row {
            print!("{}", ch);
        }
        println!();
    }
    println!();
    let mut sum : u32 = 0;
    for i in 0..matrix.len() {
        let mut j = 0;
        while j < matrix[0].len() {
            if matrix[i][j].is_digit(10) {
                let mut num_str = String::new();
                let mut is_marked = false;
                // The order of conditionsin the while is very important!
                while j < matrix[0].len() && matrix[i][j].is_digit(10) {
                    if mark_matrix[i][j] == 'x' {
                        is_marked = true;
                    }
                    num_str.push(matrix[i][j]);
                    j += 1;
                }
                print!("Found number {}. ", num_str);
                if is_marked {
                    println!("Its marked, adding to sum.");
                    sum += num_str.parse::<u32>().unwrap();
                }
                else {
                    println!("It's unmarked, ignoring.");
                }
            }
            else {
                j += 1;
            }
        }
    }
    return sum;
}

fn part2_solution(matrix: &Vec<Vec<char>>) -> u32 {
    let mut mark_matrix = vec![vec![0i32; matrix.len()]; matrix[0].len()];
    let mut mark_counter : i32 = 1;
    let mut map : HashMap<i32, Vec<i32>> = HashMap::new();

    for (i, line) in matrix.iter().enumerate() {
        for (j, &ch) in line.iter().enumerate() {
            if !ch.is_digit(10) && ch != '.' {
                let mut assign_num = -1;
                if ch == '*' {
                    assign_num = mark_counter;
                    mark_counter += 1;
                    map.insert(mark_counter, Vec::new());
                }
                mark_matrix[i][j] = assign_num;
                if i > 0 {
                    mark_matrix[i - 1][j] = assign_num; // Up
                    if j > 0 { mark_matrix[i - 1][j - 1] = assign_num; } // Diagonal Up-Left
                    if j < matrix[0].len() - 1 { mark_matrix[i - 1][j + 1] = assign_num; } // Diagonal Up-Right
                }
                if j > 0 { mark_matrix[i][j - 1] = assign_num; } // Left
                if i < matrix.len() - 1 {
                    mark_matrix[i + 1][j] = assign_num; // Down
                    if j > 0 { mark_matrix[i + 1][j - 1] = assign_num; } // Diagonal Down-Left
                    if j < matrix[0].len() - 1 { mark_matrix[i + 1][j + 1] = assign_num; } // Diagonal Down-Right
                }
                if j < matrix[0].len() - 1 { mark_matrix[i][j + 1] = assign_num; } // Right
            }
        }
    }

    for row in &mark_matrix {
        for &ch in row {
            if ch == -1 {
                print!(".");
            }
            else {
                print!("{}", ch);
            }
        }
        println!();
    }


    let mut sum : u32 = 0;
    for i in 0..matrix.len() {
        let mut j = 0;
        while j < matrix[0].len() {
            if matrix[i][j].is_digit(10) {
                let mut num_str = String::new();
                let mut is_marked = false;
                let mut mark_mat_val = 0;
                // The order of conditionsin the while is very important!
                while j < matrix[0].len() && matrix[i][j].is_digit(10) {
                    if mark_matrix[i][j] != 0 {
                        is_marked = true;
                        mark_mat_val = mark_matrix[i][j];
                    }
                    num_str.push(matrix[i][j]);
                    j += 1;
                }
                print!("Found number {}. ", num_str);
                if is_marked {
                    let num = num_str.parse::<i32>().unwrap();
                    if mark_mat_val != -1 {
                        println!("{} - aaaa - ", mark_mat_val);
                        map.entry(mark_mat_val).or_insert_with(Vec::new).push(num);
                    }
                    println!("Its marked, adding to sum.");
                }
                else {
                    println!("It's unmarked, ignoring.");
                }
            }
            else {
                j += 1;
            }
        }
    }
    let mut answ_sum : i32 = 0;
    for item in map {
        println!("{} {}", item.0, item.1.len());
        if item.1.len() == 2 {
            let product : i32 = item.1.iter().product();
            println!("{}", product);
            answ_sum += product;
        }
    }
    return answ_sum as u32;
}

fn main() {
    let mut input = fs::File::open("input.txt").unwrap();
    let mut contents = String::new();

    let _ = input.read_to_string(&mut  contents);
    
    let matrix : Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    
    println!("SUM: {}", part2_solution(&matrix));

}
