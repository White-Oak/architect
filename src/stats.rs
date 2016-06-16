use diff::Stat;

use std::collections::*;
use chrono::naive::datetime::NaiveDateTime;
use chrono::datetime::DateTime;
use chrono::offset::fixed::FixedOffset;
use chrono::{Datelike, Timelike};

pub fn process(mut stats: Vec<Stat>) -> BTreeMap<String, ResultStat> {
    let mut gathered: BTreeMap<String, ResultStat> = BTreeMap::new();
    let mut total = ResultStat::new("TOTAL".into(), "TOTAL".into());
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
        increaser(&mut total.stat);
        let secs = stat.time.seconds();
        let naive_dt = NaiveDateTime::from_timestamp(secs, 0);
        let tz = FixedOffset::east(stat.time.offset_minutes() * 60);
        let dt: DateTime<FixedOffset> = DateTime::from_utc(naive_dt, tz);
        let weekday = dt.weekday().num_days_from_monday() as usize;
        increaser(&mut s.days[weekday]);
        increaser(&mut total.days[weekday]);
        // 0 - 6 = night
        // 7 - 12 = morning
        // 13 - 18 = day
        // 19 - 24 = evening
        let daytime = (dt.hour() / 6) as usize;
        increaser(&mut s.daytimes[daytime]);
        increaser(&mut total.daytimes[daytime]);
    }
    gathered.insert("TOTAL".into(), total);
    gathered
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct ResultStat {
    pub author: String,
    pub email: String,
    pub stat: MainStat,
    pub days: [MainStat; 7],
    pub daytimes: [MainStat; 4],
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Copy, Clone, Default)]
pub struct MainStat {
    pub inserts: u32,
    pub dels: u32,
    pub commits: u32,
}

impl ResultStat {
    pub fn new(author: String, email: String) -> Self {
        ResultStat {
            author: author,
            email: email,
            stat: MainStat::default(),
            days: [MainStat::default(); 7],
            daytimes: [MainStat::default(); 4],
        }
    }
}
