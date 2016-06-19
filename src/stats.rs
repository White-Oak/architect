use diff::Stat;

use std::collections::*;
use chrono::*;
use git2::Time;
use lazysort::SortedBy;

fn dt_from_gittime(time: &Time) -> DateTime<FixedOffset> {
    let secs = time.seconds();
    let naive_dt = NaiveDateTime::from_timestamp(secs, 0);
    let tz = FixedOffset::east(time.offset_minutes() * 60);
    DateTime::from_utc(naive_dt, tz)
}

pub fn process(stats: Vec<Stat>) -> AllResultStat {
    AllResultStat{
        top_monthly: calculate_top_contributers_per_month(&stats),
        common_stats: get_more_author_stats(&stats)
    }
}

fn get_more_author_stats(stats: &[Stat]) -> BTreeMap<String, ResultStat> {
    let mut gathered: BTreeMap<String, ResultStat> = BTreeMap::new();
    let mut total = ResultStat::new("TOTAL".into(), "TOTAL".into());
    for stat in stats {
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
        let dt = dt_from_gittime(&stat.time);
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

fn get_authors_stats(stats: &[&Stat]) -> HashMap<Author, MainStat> {
    let mut gathered: HashMap<Author, MainStat> = HashMap::new();
    for stat in stats {
        let author = Author(stat.author.clone(), stat.email.clone());
        if !gathered.contains_key(&author) {
            let new_stat = MainStat::default();
            gathered.insert(author.clone(), new_stat);
        }
        let mut s = gathered.get_mut(&author).unwrap();

        // A capturing closure that increases statistics for a selected stat
        let increaser = |s: &mut MainStat| {
            s.inserts += stat.inserts;
            s.dels += stat.dels;
            s.commits += 1;
        };
        increaser(s);
    }
    gathered
}

fn calculate_top_contributers_per_month(stats: &[Stat]) -> Vec<TopMonthContributer> {
    let mut now = Local::now();
    let mut map: HashMap<DateTime<Local>, Vec<&Stat>> = HashMap::new();
    fn same_month<T: Datelike, T2: Datelike>(dt: T, dt2: T2) -> bool {
        dt.year() == dt2.year() && dt.month0() == dt2.month0()
    }
    while map.values().map(|vec| vec.len()).sum::<usize>() < stats.len() {
        let filtered: Vec<&Stat> = stats.iter().filter(|i| same_month(dt_from_gittime(&i.time), now)).collect();
        map.insert(now, filtered);
        let mut year = now.year();
        let month = if let Some(m) = now.month0().checked_sub(1) {
            m
        } else {
            year -= 1;
            11
        };
        now = now.with_year(year).unwrap().with_month0(month).unwrap();
    }
    let mut result = Vec::new();
    for (dt, vec) in map.into_iter() {
        if let Some(top_contributer) = get_authors_stats(&vec).iter().max_by_key(|&(_, i)| i.commits){
            result.push(TopMonthContributer::new(dt.year() as u16, dt.month0() as u8,
            top_contributer.0.clone(), *top_contributer.1));
        }
    }
    result.into_iter().sorted_by(|b, a| (a.year * 12 + a.month as u16).cmp(&(b.year * 12 + a.month as u16))).collect()
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct TopMonthContributer {
    pub year: u16,
    pub month: u8,
    pub author: Author,
    pub stat: MainStat
}

impl TopMonthContributer{
    fn new(year: u16, month: u8, author: Author, stat: MainStat) -> Self{
        TopMonthContributer {
            year: year,
            month: month,
            author: author,
            stat: stat
        }
    }
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct AllResultStat {
    pub top_monthly: Vec<TopMonthContributer>,
    pub common_stats: BTreeMap<String, ResultStat>,
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Author (pub String, pub String);

#[derive(RustcDecodable, RustcEncodable)]
pub struct ResultStat {
    pub sign: Author,
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
            sign: Author(author, email),
            stat: MainStat::default(),
            days: [MainStat::default(); 7],
            daytimes: [MainStat::default(); 4],
        }
    }
}
