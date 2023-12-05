use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashSet;



struct Card<'a> {
    card_str: &'a str,
    bar_ind: usize,
    colon_ind: usize,
}

impl<'a> Card<'a> {
    fn new(card_str: &'a str) -> Self {
        let bar_ind = card_str.find("|").expect("No '|' found");
        let colon_ind = card_str.find(":").expect("no ':' found");
        Card {card_str, bar_ind, colon_ind}
    }

    fn get_winners(&self) -> HashSet<u16> {
        let winners_str: &str = &self.card_str.clone()[self.colon_ind+1..self.bar_ind].trim();
        let winners_split: Vec<&str> = winners_str.split_whitespace().collect();

        let v: Vec<u16> = winners_split
            .iter()
            .map(|x| x.parse().unwrap())
            .collect();
        let set_out = HashSet::from_iter(v);
        set_out
    }

    fn get_mine(&self) -> HashSet<u16> {
        let mine_str: &str = &self.card_str.clone()[self.bar_ind+1..].trim();
        let mine_split: Vec<&str> = mine_str.split_whitespace().collect();

        let v: Vec<u16> = mine_split
            .iter()
            .map(|x| x.parse().unwrap())
            .collect();
        let set_out = HashSet::from_iter(v);
        set_out
    }

    fn get_score(&self) -> u16 {
        let winners = self.get_winners();
        let mine = self.get_mine();
        let inter: Vec<_> = winners.intersection(&mine).collect();
        let matches: u16 = inter.len() as u16;
        let mut score: u16 = 0;
        if matches > 0 {
            score = 1;
            for _ in 1..matches {
                score = score * 2;
            }
        }
        score
    }

}

fn main() {
    let path = Path::new("../files/day4.txt");
    let mut file = File::open(&path).expect("File not found");
    let mut data_string = String::new();
    file.read_to_string(&mut data_string).unwrap();
    let data_vec: Vec<&str> = data_string.lines().collect();
    
    let mut total_score: u16 = 0;
    for c in data_vec.iter() {
        let card = Card::new(c);
        total_score += card.get_score();
    }
    println!("Total = {}", total_score);

}
