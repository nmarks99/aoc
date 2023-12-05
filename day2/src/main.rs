#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

const N_RED: u8 = 12;
const N_GREEN: u8 = 13;
const N_BLUE: u8 = 14;


fn get_id_from_line(line: &str) -> u16 {
    
    let mut id_string = String::new();
    let colon_ind = line.find(":").unwrap();
    for i in 5..colon_ind {
        id_string.push(line.chars().nth(i).unwrap());
    }
    let id: u16 = id_string.parse().unwrap();
    id
}

fn get_count_for_color(turn: &str, color_ind: usize) -> u8 {
    
    let mut i: usize = color_ind;
    let mut comma_ind: usize;
    // println!("turn = {}", turn);
    // println!("Color ind = {}", color_ind);
    loop {
        let c = turn.chars().nth(i).unwrap();
        // println!("char[{}] = {}", i, c);
        if c.eq(&',') || i == 0 {
            comma_ind = i;
            // println!("stop at {}", i);
            break;
        }
        else {
            i = i - 1;
        }
    }
    let mut num_string = String::new();
    if comma_ind != 0 {
        comma_ind += 1;
    }
    for j in comma_ind..color_ind {
        num_string.push(turn.chars().nth(j).unwrap());
    }
    
    // println!("num_string = {:?}", num_string);
    let count: u8 = num_string.trim().parse().unwrap();
    // println!("count = {}", count);
    count
}

fn main() {
    
    let path = Path::new("./files/day2.txt");
    let mut file = File::open(&path).unwrap();
    let mut data_string = String::new();
    file.read_to_string(&mut data_string).unwrap();
    
    let lines: Vec<String> = data_string
        .lines()
        .map(|c| c.to_string())
        .collect();
   
    
    let colors = vec!["red", "green", "blue"];
    let mut total: u16 = 0;

    for game in &lines {
        // println!("---------------------------");
        // println!("{}", game);
        let id: u16 = get_id_from_line(&game);
        // println!("id = {}", id);
        let colon_ind = game.find(":").unwrap();
        let game_str = &game[colon_ind+1..];
        let split_game: Vec<&str> = game_str.split(";").collect();

        let mut possible = false;
        for turn in &split_game {
            // println!("Turn = {}", turn);
            let mut red_count: u8 = 0;
            let mut green_count: u8 = 0;
            let mut blue_count: u8 = 0;
            for color in &colors {
                let ind: Vec<_> = turn.match_indices(*color).collect();
                for (i, v) in &ind {
                    // println!("{}, {}", i, v);
                    let value: u8 = get_count_for_color(&turn, *i);
                    match v {
                        &"red" => red_count = value,
                        &"green" => green_count = value,
                        &"blue" => blue_count = value,
                        _ => panic!("ahhh")
                    };
                }
            }
            if red_count > N_RED || green_count > N_GREEN || blue_count > N_BLUE {
                possible = false;
                // println!("Impossible");
                break;
            }
            else {
                possible = true;
                // println!("Possible");
            }
        }
        if possible {
            total = total + id;
        }
    }
    println!("total = {}", total);
}
