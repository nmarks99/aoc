#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

const N_RED: u8 = 12;
const N_GREEN: u8 = 13;
const N_BLUE: u8 = 14;

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
        // let mut game: &str = &lines[0];
        let id = game.chars().nth(5).unwrap().to_digit(10).unwrap() as u16;
        println!("Game id = {}", id);
        let game_str = &game[8..];
        // println!("{}", game1);
        let split_game: Vec<&str> = game_str.split(";").collect();

        let mut possible = true;
        for turn in &split_game {
            let mut red_count: u8 = 0;
            let mut green_count: u8 = 0;
            let mut blue_count: u8 = 0;
            // println!("Turn = {}", turn);
            for color in &colors {
                let ind: Vec<_> = turn.match_indices(*color).collect();
                for (i, v) in &ind {
                    let value: u8 = turn.chars().nth(*i-2).unwrap().to_digit(10).unwrap() as u8;
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
                break;
            }
        }
        if possible {
            total = id as u16;
        }
    }
    println!("total = {}", total);
}
