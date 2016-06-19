#![cfg(all(feature = "cli", not(feature = "qt"), not(feature = "html")))]
use std::collections::*;
use ansi_term::Colour::{Green, Red, Yellow};
use lazysort::SortedBy;

use super::super::stats::*;

pub fn output(gathered: &BTreeMap<String, ResultStat>) {
    // Create a sorted iterator of statistics
    let iter = gathered.values().sorted_by(|b, a| a.stat.commits.cmp(&b.stat.commits));
    for stat in iter {
        fn print_main_stats(stats: &[MainStat]) {
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
}
