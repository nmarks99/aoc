use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("../files/day6_part2.txt").expect("File not found");
    let mut file_string = String::new();
    file.read_to_string(&mut file_string).unwrap();

    let mut lines = file_string.lines();
    let time_string: String = lines
        .next()
        .unwrap()
        .split_whitespace()
        .collect();
    let time_vec: Vec<u64> = time_string
        .split(":")
        .skip(1)
        .filter_map(|i| i.parse().ok())
        .collect();

    let record_string: String = lines
        .next()
        .unwrap()
        .split_whitespace()
        .collect();
    let record_vec: Vec<u64> = record_string
        .split(":")
        .skip(1)
        .filter_map(|i| i.parse().ok())
        .collect();

    let mut n_winners_list: Vec<u64> = vec![];
    for (t, d) in time_vec.iter().zip(record_vec.iter()) {
        let mut n_winners = 0;
        for i in 0..*t + 1 {
            let speed = i;
            let travel_time = t - i;
            let distance = speed * travel_time;
            if distance > *d {
                n_winners += 1;
            }
        }
        n_winners_list.push(n_winners);
    }

    println!("{:?}", n_winners_list);
    let total: u64 = n_winners_list.into_iter().reduce(|a, b| a * b).unwrap();
    println!("Total = {}", total);
}
