#![feature(question_mark)]
extern crate git2;
extern crate ansi_term;
extern crate chrono;
extern crate lazysort;
extern crate rustc_serialize;

mod diff;

use diff::*;

use std::collections::*;
use ansi_term::Colour::{Green, Red, Yellow};
use chrono::naive::datetime::NaiveDateTime;
use chrono::datetime::DateTime;
use chrono::offset::fixed::FixedOffset;
use chrono::{Datelike, Timelike};
use lazysort::SortedBy;
use rustc_serialize::json;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut stats = gather_stats().unwrap();
    let mut gathered: BTreeMap<String, ResultStat> = BTreeMap::new();
    for stat in &mut stats {
        if !gathered.contains_key(&stat.email) {
            let new_stat = ResultStat::new(stat.author.clone(), stat.email.clone());
            gathered.insert(stat.email.clone(), new_stat);
        }
        let mut s = gathered.get_mut(&stat.email).unwrap();

        // A capturing closure that increases statistics for a selected stat
        let increaser = |s: &mut MainStat| {
            s.inserts += stat.inserts;
            s.dels += stat.dels;
            s.commits += 1;
        };
        increaser(&mut s.stat);
        let secs = stat.time.seconds();
        let naive_dt = NaiveDateTime::from_timestamp(secs, 0);
        let tz = FixedOffset::east(stat.time.offset_minutes() * 60);
        let dt: DateTime<FixedOffset> = DateTime::from_utc(naive_dt, tz);
        let weekday = dt.weekday().num_days_from_monday() as usize;
        increaser(&mut s.days[weekday]);
        // 0 - 6 = night
        // 7 - 12 = morning
        // 13 - 18 = day
        // 19 - 24 = evening
        let daytime = (dt.hour() / 6) as usize;
        increaser(&mut s.daytimes[daytime]);
    }

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

#[derive(RustcDecodable, RustcEncodable)]
pub struct ResultStat {
    pub author: String,
    pub email: String,
    pub stat: MainStat,
    pub days: [MainStat; 7],
    pub daytimes: [MainStat; 4]
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Copy, Clone, Default)]
pub struct MainStat{
    pub inserts: u32,
    pub dels: u32,
    pub commits: u32,
}

impl ResultStat {
    pub fn new(author: String, email: String) -> Self {
        ResultStat{
            author: author,
            email: email,
            stat: MainStat::default(),
            days: [MainStat::default(); 7],
            daytimes: [MainStat::default(); 4]
        }
    }
}
