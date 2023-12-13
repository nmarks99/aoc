use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("../files/day6.txt").expect("File not found");
    let mut file_string = String::new();
    file.read_to_string(&mut file_string).unwrap();

    let mut lines = file_string.lines();
    let times: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let records: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    let mut n_winners_list: Vec<u32> = vec![];
    for (t, d) in times.iter().zip(records.iter()) {
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
    let total: u32 = n_winners_list.into_iter().reduce(|a, b| a * b).unwrap();
    println!("Total = {}", total);
}
