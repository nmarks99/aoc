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
    loop {
        let c = turn.chars().nth(i).unwrap();
        if c.eq(&',') || i == 0 {
            comma_ind = i;
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
    
    let count: u8 = num_string.trim().parse().unwrap();
    count
}

fn main() {
    
    let path = Path::new("../files/day2.txt");
    let mut file = File::open(&path).unwrap();
    let mut data_string = String::new();
    file.read_to_string(&mut data_string).unwrap();
    
    let lines: Vec<String> = data_string
        .lines()
        .map(|c| c.to_string())
        .collect();
   
    
    let colors = vec!["red", "green", "blue"];
    let mut total: u16 = 0;
    let mut total_power: u32 = 0;

    for game in &lines {
        println!("{}", game);

        let mut red_count_game: Vec<u16> = vec![];
        let mut green_count_game: Vec<u16> = vec![];
        let mut blue_count_game: Vec<u16> = vec![];

        let id: u16 = get_id_from_line(&game);
        let colon_ind = game.find(":").unwrap();
        let game_str = &game[colon_ind+1..];
        let split_game: Vec<&str> = game_str.split(";").collect();

        let mut possible = false;
        for turn in &split_game {
            let mut red_count_turn: u8 = 0;
            let mut green_count_turn: u8 = 0;
            let mut blue_count_turn: u8 = 0;
            for color in &colors {
                let ind: Vec<_> = turn.match_indices(*color).collect();
                for (i, v) in &ind {
                    let value: u8 = get_count_for_color(&turn, *i);
                    match v {
                        &"red" => red_count_turn = value,
                        &"green" => green_count_turn = value,
                        &"blue" => blue_count_turn = value,
                        _ => panic!("ahhh")
                    };
                }
            }
            red_count_game.push(red_count_turn.into());
            green_count_game.push(green_count_turn.into());
            blue_count_game.push(blue_count_turn.into());
            // if red_count_turn > N_RED || green_count_turn > N_GREEN || blue_count_turn > N_BLUE {
            //     possible = false;
            //     break;
            // }
            // else {
            //     possible = true;
            // }
        }
        // if possible {
        //     total = total + id;
        // }
         
        println!("Red = {:?}", red_count_game);
        println!("Green = {:?}", green_count_game);
        println!("Blue = {:?}", blue_count_game);
        let max_red: u16 = *red_count_game.iter().max().expect("hmm1") as u16;
        let max_green: u16 = *green_count_game.iter().max().expect("hmm2") as u16;
        let max_blue: u16 = *blue_count_game.iter().max().expect("hmm3") as u16;
        println!("Max red = {}", max_red);
        println!("Max green = {}", max_green);
        println!("Max blue = {}", max_blue);
        let power = max_red * max_green * max_blue;
        println!("Power =  {}", power);
        total_power += power as u32;
        println!("Total power = {}" ,total_power);
        println!("--------------------------------");

    }
    println!("\nTotal power = {}", total_power);
}
