//
// entry.rs
// Copyright (C) 2019 Lucas Costa Campos <rmk236@gmail.com>
// Distributed under terms of the MIT license.
//

use std::collections::HashMap;
use rand::{thread_rng, Rng};
use rand::rngs::{ThreadRng};

const PREFIX_SIZE: usize = 5;

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

#[derive(Clone, Debug)]
pub struct FollowUp{
    pub possibilities: HashMap<String, f64>
}

impl FollowUp{

    pub fn new() -> Self {
        FollowUp{possibilities: HashMap::new()}
    }

    pub fn normalize(mut self) {

        let total = self.possibilities.values().fold(0.0, |a, b| a + b);

        for p in self.possibilities.values_mut() {
            *p = *p/total;
        }

    }

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


#[derive(Clone, Debug)]
pub struct MarkovData {
    pub data: HashMap<Prefix, FollowUp>,
    rng: ThreadRng
}

impl MarkovData {

    pub fn new(text: &String) -> Self{

        let mut loc_text = text.clone();
        loc_text.make_ascii_lowercase();

        let mut data: HashMap<Prefix, FollowUp> = HashMap::new();
        let splitted: Vec<&str> = loc_text.split_whitespace().collect();
        let len_c = splitted.len() - PREFIX_SIZE;

        for i in 0..len_c {
            // let p = Prefix::new([splitted[i].to_string(), splitted[i+1].to_string()]);
            let mut words: [String; PREFIX_SIZE] = Default::default();
            for idx in 0..PREFIX_SIZE {
                words[idx] = splitted[i+idx].to_string();
            }

            let p = Prefix::new(words);

            let here = data.entry(p).or_insert(FollowUp::new());
            let count = here.possibilities.entry(splitted[i+PREFIX_SIZE].to_string()).or_insert(0.0);
            *count += 1.0;
        }

        for fu in data.values_mut() {

            let total = fu.possibilities.values().fold(0.0, |a, b| a + b);

            for p in fu.possibilities.values_mut() {
                *p = *p/total;
            }

        }

        MarkovData{data, rng: thread_rng()}

    }

    pub fn choose_next(&mut self, p: &Prefix) -> String {
        let d = &self.data[p];
        d.choose_next(self.rng.gen_range(0.0, 1.0))
    }

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
