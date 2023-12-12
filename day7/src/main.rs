// Learning rust by doing advent of code
// https://adventofcode.com/2023/day/7
// Vahe Danielyan, 2023

use std::collections::HashMap;
use std::{fs, cmp};
use std::io::prelude::*;

fn max_num_repeat(hand: &str) -> (i32, char) {
    let mut char_counts = HashMap::new();
    let mut max_count = 0;
    let mut max_char = '\0';

    for c in hand.chars() {
        //if c == 'J' { for part 2
        //    continue;
        //}
        let count = char_counts.entry(c).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            max_char = c;
        }
    }
    (max_count, max_char)
}

fn num_j(hand: &str) -> i32 {
    let mut answ = 0;
    for c in hand.chars() {
        if c == 'J' {
            answ += 1;
        }
    }
    return answ;
}

fn card_to_cost(card : &char) -> i32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11, // 1 for part 2
        'T' => 10,
        '2'..='9' => card.to_digit(10).unwrap() as i32,
        _ => -1,
    }
}

fn hand_to_cost(hand : &str) -> i32 {
    let max_repeat = max_num_repeat(hand);
    let max_repeat_cnt = max_repeat.0 + num_j(hand);
    let max_repeat_char = max_repeat.1;
    if max_repeat_cnt == 5 {
        return 7;
    }
    if max_repeat_cnt == 4 {
        return 6; 
    }
    if max_repeat_cnt == 3 {
        //let max_repeat_2 = max_num_repeat(&hand.replace(max_repeat_char, "").replace('J', ""));
        let max_repeat_2 = max_num_repeat(&hand.replace(max_repeat_char, ""));
        if max_repeat_2.0 == 2 {
            return 5;
        }
        return 4;
    }
    if max_repeat_cnt == 2 {
        //let max_repeat_2 = max_num_repeat(&hand.replace(max_repeat_char, "").replace('J', ""));
        let max_repeat_2 = max_num_repeat(&hand.replace(max_repeat_char, ""));
        if max_repeat_2.0 == 2 {
            return 3;
        }
        return 2;
    }
    return 1;
}

fn compare_cards(card1 : &char, card2 : &char) -> cmp::Ordering {
    if card1 == card2 {
        return cmp::Ordering::Equal; 
    }
    return card_to_cost(card1).cmp(&card_to_cost(card2));
}

fn compare(hand1 : &str, hand2 : &str) -> cmp::Ordering {
    if hand1 == hand2 {
        println!("EQQQQQQQQQQQQ");
        return cmp::Ordering::Equal;
    }

    let hand1_cost = hand_to_cost(hand1);
    let hand2_cost = hand_to_cost(hand2);

    if hand1_cost != hand2_cost {
        if hand1_cost < hand2_cost {
            return cmp::Ordering::Less;
        }
        return cmp::Ordering::Greater;
    }
    let h1_chars : Vec<char> = hand1.chars().collect();
    let h2_chars : Vec<char>  = hand2.chars().collect();

    for i in 0..h1_chars.len() {
        if h1_chars[i] != h2_chars[i] {
            return compare_cards(&h1_chars[i], &h2_chars[i]);
        }
    }

    return cmp::Ordering::Equal;
}

fn main() {
    let mut file = fs::File::open("input.txt").unwrap();
    let mut contents : String = String::new();

    let _ = file.read_to_string(&mut contents);
    
    let mut pairs : Vec<(String, i64)>;
    pairs = contents
        .lines() .map(|line| { let parts : Vec<&str> = line.split_whitespace().collect();
            let number = parts[1].parse::<i64>().unwrap();
            (parts[0].to_string(), number)
        })
        .collect();

    pairs.sort_by(|a, b| compare(&a.0, &b.0) );
    println!("SORTED");

    let mut rank : i64 = 1;
    let mut answ : i64 = 0;
    for pair in &pairs {
        println!("{} {} {}-{}", pair.0, pair.1 * rank, rank, pair.1);
        answ += pair.1 * rank;
        rank += 1;
    }
    println!("{}", answ);
}
