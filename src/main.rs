mod entry;
use entry::{MarkovData};
use std::fs;

use clap::{Arg, App};

fn main() {

    let matches = App::new("Markov text generator")
        .version("0.1.0")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("File with the text the Markov generator should learn from")
        )
        .arg(Arg::with_name("words")
            .short("w")
            .long("words")
            .takes_value(true)
            .help("Number of words that should be generated")
        )
        .get_matches();

    let filename = matches.value_of("file").unwrap_or("juliet.txt");
    let contents = fs::read_to_string("./juliet.txt").expect("Something went wrong reading the file");

    let n_words_str = matches.value_of("words").unwrap_or("100");
    let n_words = n_words_str.parse::<i32>().unwrap();

    let mut markov = MarkovData::new(&contents.to_string());
    let mut prefix = markov.choose_seed();

    for _ in 0..n_words {

        let word = markov.choose_next(&prefix);
        print!("{} ", word);
        prefix.push_new(word);

    }

    println!("");

}
