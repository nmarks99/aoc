use std::fs;
use std::collections::HashMap;
use std::hash::Hash;

fn word_nums(msg: &String) -> u32 {

    let mut word_nums: HashMap<&str, u8> = HashMap::new();
    word_nums.insert("one", 1);
    word_nums.insert("two", 2);
    word_nums.insert("three", 3);
    word_nums.insert("four", 4);
    word_nums.insert("five", 5);
    word_nums.insert("six", 6);
    word_nums.insert("seven", 7);
    word_nums.insert("eight", 8);
    word_nums.insert("nine", 9);
    
    // let mut string_out = String::new();
    let mut map1: HashMap<usize, u8> = HashMap::new();
    let mut map2: HashMap<usize, u8> = HashMap::new();

    for k in word_nums.keys() {
        let first = msg.find(k);
        if first.is_some() {
            let val = word_nums.get(k).unwrap();
            // let length = k.len();
            let i0 = first.unwrap();
            // let i1 = i0 + length;
            map1.insert(i0, *val);
        }
    }
    
    for (i,v) in msg.chars().enumerate() {
        if v.is_numeric() {
            map2.insert(i,v.to_digit(10).unwrap() as u8);
        }
    }
    
    map1.extend(map2);
    
    let first_ind: usize = *map1.keys().min().unwrap() as usize;
    let last_ind: usize = *map1.keys().max().unwrap() as usize;
    let first_val: u32 = map1[&first_ind].into();
    let last_val: u32 = map1[&last_ind].into();

    let mut str_val = String::new();
    str_val.extend(first_val.to_string().as_str().chars());
    str_val.extend(last_val.to_string().as_str().chars());
    let val_int: u32 = str_val.parse().expect("Cannot convert to str_val to u32");
    val_int
}

// fn get_line_val(msg: &String) -> u32 {
//
//     let mut first: Option<char> = None;
//     let mut last: Option<char> = None;
//     for c in msg.chars() {
//         if c.is_numeric() {
//             if first.is_none() {
//                 first = Some(c);
//             }
//             else {
//                 last = Some(c);
//             }
//         }
//     }
//     if first.is_none() {
//         return 0u32;
//     }
//     if last.is_none() {
//         last = first;
//     }
//     
//     let mut str_val = String::new();
//     str_val.push(first.expect("No first"));
//     str_val.push(last.expect("No last"));
//     let val_int: u32 = str_val.parse().expect("Cannot convert to str_val to u32");
//     val_int
// }

fn main() {


    let file_path: &str = "./files/day1.txt";
    let cont : Vec<u8> = fs::read(file_path).expect("Could not read file");

    let mut line: String = Default::default();
    let mut data: Vec<String> = vec![];
    for i in 0..cont.len() {
        let c = cont[i] as char;
        if c != '\n' {
            line.push(c);
        }
        else {
            data.push(line.clone());
            line.clear();
        }
    }

    let mut total: u32 = 0;
    for li in data {
        println!("line: {}", li);
        let v: u32 = word_nums(&li);
        // let v: u32 = get_line_val(&li);
        println!("val = {}", v);
        total = total + v;
    }

    println!("\nTotal = {}", total);

}
