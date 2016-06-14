#![feature(question_mark)]
extern crate git2;
extern crate ansi_term;
extern crate chrono;

mod diff;
mod output;

use diff::*;

use std::collections::*;
use ansi_term::Colour::{Green, Red, Yellow};
use chrono::naive::datetime::NaiveDateTime;
use chrono::datetime::DateTime;
use chrono::offset::fixed::FixedOffset;
use chrono::{Datelike, Timelike};

fn main() {
    let mut stats = gather_stats().unwrap();
    let mut gathered: BTreeMap<String, ResultStat> = BTreeMap::new();
    for stat in &mut stats {
        if !gathered.contains_key(&stat.author) {
            let mut new_stat = ResultStat::new(stat.author.clone());
            gathered.insert(stat.author.clone(), new_stat);
        }
        let mut s = gathered.get_mut(&stat.author).unwrap();
        s.inserts += stat.inserts;
        s.dels += stat.dels;
        s.commits += 1;
        let secs = stat.time.seconds();
        let naive_dt = NaiveDateTime::from_timestamp(secs, 0);
        let tz = FixedOffset::east(stat.time.offset_minutes() * 60);
        let dt: DateTime<FixedOffset> = DateTime::from_utc(naive_dt, tz);
        let weekday = dt.weekday().num_days_from_monday() as usize;
        let mut day_stat = s.days[weekday];
        day_stat.inserts += stat.inserts;
        day_stat.dels += stat.dels;
        day_stat.commits += 1;
        s.days[weekday] = day_stat;
        // 0 - 6 = night
        // 7 - 12 = morning
        // 13 - 18 = day
        // 19 - 24 = evening
        let daytime = (dt.hour() / 6) as usize;
        let mut daytime_stat = s.daytimes[daytime];
        daytime_stat.inserts += stat.inserts;
        daytime_stat.dels += stat.dels;
        daytime_stat.commits += 1;
        s.daytimes[daytime] = daytime_stat;
    }

    for stat in gathered.values(){
        println!("Statistics for {}", stat.author);
        println!("Commits: {}; Insertions: {}; Deletions: {}",
            Yellow.paint(stat.commits.to_string()),
            Green.paint(stat.inserts.to_string()),
            Red.paint(stat.dels.to_string()));
        println!("Days\t\tMon\tTue\tWed\tThu\tFri\tSat\tSun");
        print!("Commits\t");
        for i in 0..7 {
            print!("\t{}", Yellow.paint(stat.days[i].commits.to_string()));
        }
        println!("");
        print!("Insertions");
        for i in 0..7 {
            print!("\t{}", Green.paint(stat.days[i].inserts.to_string()));
        }
        println!("");
        print!("Deletions");
        for i in 0..7 {
            print!("\t{}", Red.paint(stat.days[i].dels.to_string()));
        }
        println!("");
        println!("");
        println!("Daytime\t\tNight\tMorning\tDay\tEvening");
        print!("Commits\t");
        for i in 0..4 {
            print!("\t{}", Yellow.paint(stat.daytimes[i].commits.to_string()));
        }
        println!("");
        print!("Insertions");
        for i in 0..4 {
            print!("\t{}", Green.paint(stat.daytimes[i].inserts.to_string()));
        }
        println!("");
        print!("Deletions");
        for i in 0..4 {
            print!("\t{}", Red.paint(stat.daytimes[i].dels.to_string()));
        }
        println!("");
        println!("");
    }
}

pub struct ResultStat {
    pub author: String,
    pub inserts: u32,
    pub dels: u32,
    pub commits: u32,
    pub days: [MainStat; 7],
    pub daytimes: [MainStat; 4]
}

#[derive(Copy, Clone)]
pub struct MainStat{
    pub inserts: u32,
    pub dels: u32,
    pub commits: u32,
}

impl MainStat {
    pub fn new() -> Self {
        MainStat{inserts: 0, dels: 0, commits: 0}
    }
}
impl ResultStat {
    pub fn new(author: String) -> Self {
        ResultStat{
            author: author,
            inserts: 0,
            dels: 0,
            commits: 0,
            days: [MainStat::new(); 7],
            daytimes: [MainStat::new(); 4]
        }
    }
}
