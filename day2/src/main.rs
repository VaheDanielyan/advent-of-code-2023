// Learning rust by doing advent of code
// https://adventofcode.com/2023/day/2
// Vahe Danielyan, 2023

use std::io::{prelude::*, self};
use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    
    let file = match File::open(&path) {
        Err(why) => panic!("AAA {}", why),
        Ok(file) => file,
    };
    
    let (red_count, green_count, blue_count) = (12, 13, 14);
    let lines : Vec<String> = io::BufReader::new(file).lines().filter_map(|line| line.ok()).collect();

    let mut answ : i32 = 0;
    let mut power_sum : i32 = 0;
    for line in lines {
        let game_data : Vec<&str> = line.split(':')
            .collect();
        let game_id : i32 = game_data[0]
            .split_whitespace()
            .collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();
        let mut condition : bool = true;

        let game_iterations : Vec<&str> = game_data[1].split(";").collect();
        let (mut red_max, mut blue_max, mut green_max) = (0, 0, 0);
        for iter in game_iterations {
            let group = iter.split(',');
            for data_point in group {
                let color = data_point.split_whitespace()
                    .collect::<Vec<&str>>()[1];
                let number : i32 = data_point.split_whitespace()
                    .collect::<Vec<&str>>()[0]
                    .parse()
                    .unwrap();
                if  color == "blue" {
                    if number > blue_max {
                        blue_max = number;
                    }
                    if number > blue_count {
                        condition = false;
                    }
                }
                if color == "red" {
                    if number > red_max {
                        red_max = number;
                    }
                    if number > red_count {
                        condition = false;
                    }
                }
                if color == "green" {
                    if number > green_max {
                        green_max = number;
                    }
                    if number > green_count {
                        condition = false;
                    }
                }
                println!("{} {} {} {}", color, number, condition, game_id);
            }

        }
        power_sum += green_max * red_max * blue_max;
        if condition == true {
            answ += game_id;
        }

    }
    println!("Answer is: {}, {}", answ, power_sum); 

}
