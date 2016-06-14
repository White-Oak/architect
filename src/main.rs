#![feature(question_mark)]
extern crate git2;
extern crate ansi_term;

mod diff;
mod output;

use diff::*;

use std::collections::*;
use ansi_term::Colour::{Green, Red};

fn main() {
    let mut stats = gather_stats().unwrap();
    let mut gathered: BTreeMap<String, ResultStat> = BTreeMap::new();
    for stat in &mut stats {
        if !gathered.contains_key(&stat.author) {
            let new_stat = ResultStat::new(stat.author.clone());
            gathered.insert(stat.author.clone(), new_stat);
        } else {
            let s = gathered.get_mut(&stat.author).unwrap();
            s.inserts += stat.inserts;
            s.dels += stat.dels;
        }
    }

    for stat in gathered.values(){
        println!("Statistics for {}", stat.author);
        println!("Insertions: {}; Deletions: {}", Green.paint(stat.inserts.to_string()),
            Red.paint(stat.dels.to_string()));
        println!("");
    }
}

pub struct ResultStat {
    pub author: String,
    pub inserts: u32,
    pub dels: u32,
    pub commits: u32,
    pub days: [(u32, u32, u32); 7]
}

impl ResultStat {
    pub fn new(author: String) -> Self {
        ResultStat{
            author: author,
            inserts: 0,
            dels: 0,
            commits: 0,
            days: [(0, 0, 0); 7]
        }
    }
}
