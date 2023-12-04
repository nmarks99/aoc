use std::fs;
use std::collections::HashMap;

fn word_to_num(msg: &String) -> String {

    let mut word_nums: HashMap<&str, u32> = HashMap::new();
    word_nums.insert("one", 1);
    word_nums.insert("two", 2);
    word_nums.insert("three", 3);
    word_nums.insert("four", 4);
    word_nums.insert("five", 5);
    word_nums.insert("six", 6);
    word_nums.insert("seven", 7);
    word_nums.insert("eight", 8);
    word_nums.insert("nine", 9);
    
    let mut string_out = msg.clone();

    for k in word_nums.keys() {
        let first = string_out.find(k);
        if first.is_some() {
            let val = word_nums.get(k).unwrap();
            let length = k.len();
            let i0 = first.unwrap();
            let i1 = i0 + length;
            string_out.replace_range(i0..i1, &val.to_string());
        }
    }
    string_out
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

    let file_path: &str = "../files/day1_2_test.txt";
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
        println!("line = {}", line);
        let line_out = word_to_num(&line);
        let v: u32 = get_num(&line_out);
        print!("line_out: {}\t", line_out);
        println!("value = {}\n", v);
        total = total + v;
    }

    println!("\nTotal = {}", total);

}
