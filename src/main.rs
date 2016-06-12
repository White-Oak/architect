#![feature(question_mark)]
extern crate git2;

mod diff;
mod output;

use diff::*;
use output::*;

use std::collections::*;

fn main() {
    let mut stats = gather_stats().unwrap();
    let mut gathered: BTreeMap<String, Stat> = BTreeMap::new();
    for stat in &mut stats {
        if !gathered.contains_key(&stat.author) {
            let mut new_stat = stat.clone();
            new_stat.message = Some(format!("Statistics for {}", stat.author));
            gathered.insert(stat.author.clone(), new_stat);
        } else {
            let s = gathered.get_mut(&stat.author).unwrap();
            s.inserts += stat.inserts;
            s.dels += stat.dels;
        }
    }

    for stat in gathered.values(){
        print_stat(stat);
    }
}
