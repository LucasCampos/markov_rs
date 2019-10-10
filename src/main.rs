mod entry;
use entry::{MarkovData};
use std::fs;

fn main() {

    let contents = fs::read_to_string("./juliet.txt").expect("Something went wrong reading the file");

    let mut markov = MarkovData::new(&contents.to_string());
    let mut prefix = markov.choose_seed();

    for _ in 0..100 {

        let word = markov.choose_next(&prefix);
        print!("{} ", word);
        prefix.push_new(word);

    }

    println!("");

}
