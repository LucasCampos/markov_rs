//
// entry.rs
// Copyright (C) 2019 Lucas Costa Campos <rmk236@gmail.com>
// Distributed under terms of the MIT license.
//

use std::collections::HashMap;
use rand::{thread_rng, Rng};
use rand::rngs::{ThreadRng};

const PREFIX_SIZE: usize = 3;

///This class will contain the Prefix of our Markov chain. You can think
///of it as the memory of the graph so far. 
#[derive(Clone, Debug, Hash)]
pub struct Prefix
{
    // Prefix
    pub entries: [String; PREFIX_SIZE]
}

impl Prefix {
    pub fn new(entries: [String; PREFIX_SIZE]) -> Prefix {
        let e = entries.clone();
        Self{
            entries: e
        }
    }

    ///Update the Prefix with a new (last) word. Very useful
    ///to update it with a recently-generated word
    pub fn push_new(&mut self, word: String) {
        for i in 0..PREFIX_SIZE-1 {
            self.entries[i] = self.entries[i+1].clone();
        }
        self.entries[PREFIX_SIZE-1] = word;
    }
}

impl PartialEq for Prefix {

    fn eq(&self, other: &Prefix) -> bool {

        let mut are_eq = true;
        for i in 0..PREFIX_SIZE {
            are_eq = are_eq && self.entries[i] == other.entries[i];
        }
        are_eq
    }
}

impl Eq for Prefix {}

///Structure with the possible followups of any given
///Prefix. It has two entries, with the word, and their
///probability of being the next one. Obviously,
///the sum of all probabilities must should be 1.
#[derive(Clone, Debug)]
pub struct FollowUp{
    pub possibilities: HashMap<String, f64>
}

impl FollowUp{

    pub fn new() -> Self {
        FollowUp{possibilities: HashMap::new()}
    }

    /// Update the probabilities to be sure they 
    /// sum to one
    pub fn normalize(mut self) {

        let total = self.possibilities.values().fold(0.0, |a, b| a + b);

        for p in self.possibilities.values_mut() {
            *p = *p/total;
        }

    }

    ///Given a number chosen from a distribution, choses a
    ///random choice with weights dictated by the second 
    ///entry in the tuple.
    pub fn choose_next(&self, random_val: f64) -> String {

        let mut accum = 0.0;
        for k in self.possibilities.keys() {
            let p = &self.possibilities[k];
            accum += p;

            if random_val <= accum {
                return k.clone()
            }
        }

        "".to_string()

    }
}

///Main class of our program. This will create a graph with the Prefix->FollowUps using the text
///input.
///
///For convenience, it has an internal state due to Random Number Generator. 
#[derive(Clone, Debug)]
pub struct MarkovData {
    pub data: HashMap<Prefix, FollowUp>,
    rng: ThreadRng
}

impl MarkovData {

    pub fn new(text: &String) -> Self{

        //Make sure all parts of the text are in lower case
        let mut loc_text = text.clone();
        loc_text.make_ascii_lowercase();

        let mut data: HashMap<Prefix, FollowUp> = HashMap::new();
        
        //Split text into words
        let splitted: Vec<&str> = loc_text.split_whitespace().collect();

        //As there is no follow up for the last few words, we are not going to learn 
        //what could follow them
        let len_c = splitted.len() - PREFIX_SIZE;

        for i in 0..len_c {
            let mut words: [String; PREFIX_SIZE] = Default::default();
            for idx in 0..PREFIX_SIZE {
                words[idx] = splitted[i+idx].to_string();
            }

            let p = Prefix::new(words);

            let here = data.entry(p).or_insert(FollowUp::new());
            let count = here.possibilities.entry(splitted[i+PREFIX_SIZE].to_string()).or_insert(0.0);
            *count += 1.0;
        }

        //Normalize the followups to each Prefix to be sure they sum to one.
        for fu in data.values_mut() {

            let total = fu.possibilities.values().fold(0.0, |a, b| a + b);

            for p in fu.possibilities.values_mut() {
                *p = *p/total;
            }

        }

        MarkovData{data, rng: thread_rng()}

    }

    ///Choose next word given a Prefix
    pub fn choose_next(&mut self, p: &Prefix) -> String {
        let d = &self.data[p];
        d.choose_next(self.rng.gen_range(0.0, 1.0))
    }

    ///Randomlu choose one of the prefixes present. This is useful
    ///as a seed for the text. 
    pub fn choose_seed(&mut self) -> Prefix {
        let ran = self.rng.gen_range(0.0, 1.0);
        let each_weight = 1.0/self.data.len() as f64;
        
        let mut i = 1;
        for k in self.data.keys() {
            if ran <= (i as f64)*each_weight {
                return k.clone();
            }
            i += 1;
        }

        let words: [String; PREFIX_SIZE] = Default::default();
        Prefix::new(words)
    }
}
