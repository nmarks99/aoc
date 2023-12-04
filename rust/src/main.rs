use std::fs;
use std::collections::HashMap;

fn word_to_num(msg: &String) {

    let mut word_nums: HashMap<&str, (u32, u32)> = HashMap::new();
    word_nums.insert("one", (1,3));
    word_nums.insert("two", (2,3));
    word_nums.insert("three", (3,5));
    word_nums.insert("four", (4,4));
    word_nums.insert("five", (5,4));
    word_nums.insert("six", (6,3));
    word_nums.insert("seven", (7,5));
    word_nums.insert("eight", (8,5));
    word_nums.insert("nine", (9,4));

    for k in word_nums.keys() {
        if msg.contains(k) {
            let val = word_nums.get(k).unwrap().0;
            let length = word_nums.get(k).unwrap().1;
            println!("{} = {} of length {}",k,val,length);
        }
    }
}

fn get_num(msg: &String) -> u32 {

    let mut first: Option<char> = None;
    let mut last: Option<char> = None;
    for c in msg.chars() {
        if c.is_numeric() {
            if first.is_none() {
                first = Some(c);
            }
            else {
                last = Some(c);
            }
        }
    }
    if last.is_none() {
        last = first;
    }

    let mut str_val = String::new();
    str_val.push(first.unwrap());
    str_val.push(last.unwrap());
    let val_int: u32 = str_val.parse().unwrap();
    val_int 
}

fn main() {
    // let a = String::from("hellothree");
    // word_to_num(&a);

    let file_path: &str = "../files/day1.txt";
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
    for line in data {
        let v: u32 = get_num(&line);
        // print!("line: {}\t", line);
        // println!("value = {}", v);
        total = total + v;
    }

    println!("\nTotal = {}", total);

}
