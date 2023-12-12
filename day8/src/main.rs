// Learning rust by doing advent of code
// https://adventofcode.com/2023/day/8
// Vahe Danielyan, 2023

use std::collections::HashMap;
use std::{fs};
use std::io::prelude::*;

use num_integer::lcm;

fn multi_lcm(vect : &Vec<i32>) -> i64 {
    let mut answ : i64 = 1;
    for val in vect {
        answ = num_integer::lcm(answ as i64, val.clone() as i64); 
    }
    return answ;
}

fn find_end(tree: &HashMap<String, (String, String)>, start_node_name: String, actions : &Vec<i32>, part : String) -> i32 {
    let mut current_node_name : String = start_node_name.clone();
    let mut current_node = tree.get(&current_node_name).unwrap();

    let mut action_index = 0;
    let mut answ = 0;

    loop {
        if part == "1" {
            if current_node_name == "ZZZ" {
                break;
            }
        } else if part == "2" {
            if current_node_name.ends_with("Z") {
                break;
            }
        }
        let next_node_name;
        if actions[action_index] == 1 {
            next_node_name = current_node.0.clone();
        }
        else {
            next_node_name = current_node.1.clone(); 
        }
        current_node = tree.get(&next_node_name).unwrap();
        current_node_name = next_node_name;
        action_index += 1;
        answ += 1;
        if action_index == actions.len() {
            action_index = 0;
        }
    }
    return answ;
}

fn main() {
    let mut file = fs::File::open("input.txt").unwrap();
    let mut contents : String = String::new();
let _ = file.read_to_string(&mut contents);
    
    let actions : Vec<i32> = contents
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|char| if char == 'L' {1} else {2})
        .collect();
    
    //let mut tree : Vec<(String, (String, String))> = Vec::new();
    let mut tree : HashMap<String, (String, String)> = HashMap::new(); 
    let mut names : Vec<String> = Vec::new();
    let mut start_names : Vec<String> = Vec::new();

    for line in contents.lines().skip(2) {
        let line_vec = line
            .split('=')
            .collect::<Vec<&str>>();
        let node = line_vec[0]
            .trim()
            .to_string();

        let trimmed = line_vec[1].trim();
        let remove_parens = &trimmed[1..trimmed.len() - 1];
        let neighbors : Vec<&str> = remove_parens
            .split(',')
            .map(str::trim)
            .collect();
        tree.insert(node.clone(), (neighbors[0].to_string(), neighbors[1].to_string())); 

        if node.ends_with("A") {
            start_names.push(node.clone());
        }
        names.push(node);
    }

    let mut answ_vect : Vec<i32> = Vec::new();
    for start_node_name in &start_names {
        let answ = find_end(&tree, start_node_name.clone(), &actions, "2".to_string());
        answ_vect.push(answ);
    }

    let answ_1 = find_end(&tree, "AAA".to_string(), &actions, "1".to_string());
    println!("Answer for Part 1: {} ", answ_1);
    println!("Answer for Part 2: {} ", multi_lcm(&answ_vect));

}
