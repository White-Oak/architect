#![feature(question_mark)]
extern crate git2;
extern crate ansi_term;
extern crate chrono;
extern crate lazysort;
extern crate rustc_serialize;
extern crate num_cpus;

mod diff;
mod stats;

use stats::*;
use diff::*;

use ansi_term::Colour::{Green, Red, Yellow};
use lazysort::SortedBy;
use rustc_serialize::json;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let stats = gather_stats().unwrap();
    let gathered = process(stats);
    // Create a sorted iterator of statistics
    let iter = gathered.values().sorted_by(|b, a| {
        a.stat.commits.cmp(&b.stat.commits)
    });
    for stat in iter {
        fn print_main_stats(stats: &[MainStat]){
            print!("Commits\t");
            for stat in stats {
                print!("\t{}", Yellow.paint(stat.commits.to_string()));
            }
            println!("");
            print!("Insertions");
            for stat in stats {
                print!("\t{}", Green.paint(stat.inserts.to_string()));
            }
            println!("");
            print!("Deletions");
            for stat in stats {
                print!("\t{}", Red.paint(stat.dels.to_string()));
            }
            println!("");
            println!("");
        }
        println!("Statistics for {} <{}>", stat.author, stat.email);
        println!("Commits: {}; Insertions: {}; Deletions: {}",
            Yellow.paint(stat.stat.commits.to_string()),
            Green.paint(stat.stat.inserts.to_string()),
            Red.paint(stat.stat.dels.to_string()));
        println!("Days\t\tMon\tTue\tWed\tThu\tFri\tSat\tSun");
        print_main_stats(&stat.days);
        println!("Daytime\t\tNight\tMorning\tDay\tEvening");
        print_main_stats(&stat.daytimes);
    }

    let encoded = json::encode(&gathered).unwrap();

    let mut f = File::create("out.json").unwrap();
    f.write_all(encoded.as_bytes()).unwrap();
}
