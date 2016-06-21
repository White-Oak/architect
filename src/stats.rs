use diff::Stat;

use std::collections::*;
use chrono::*;
use git2::*;
use lazysort::SortedBy;

fn dt_from_gittime(time: &Time) -> DateTime<Local> {
    let secs = time.seconds();
    let naive_dt = NaiveDateTime::from_timestamp(secs, 0);
    let tz = FixedOffset::east(time.offset_minutes() * 60);
    DateTime::<FixedOffset>::from_utc(naive_dt, tz).with_timezone(&Local)
}

pub fn process(stats: Vec<Stat>) -> AllResultStat {
    AllResultStat {
        top_monthly: calculate_top_contributers_per_month(&stats),
        common_stats: get_more_author_stats(&stats),
        lang_stats: calc_lang_stats_at_five_points(&stats),
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
        let filtered: Vec<&Stat> =
        stats.iter().filter(|i| same_month(dt_from_gittime(&i.time), now)).collect();
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
        if let Some(top_contributer) = get_authors_stats(&vec)
        .iter()
        .max_by_key(|&(_, i)| i.commits) {
            result.push(TopMonthContributer::new(dt.year() as u16,
            dt.month0() as u8,
            top_contributer.0.clone(),
            *top_contributer.1));
        }
    }
    result.into_iter()
    .sorted_by(|b, a| cmp_dates(a.year, a.month, b.year, b.month))
    .collect()
}

fn cmp_dates(ayear: u16, amonth: u8, byear: u16, bmonth: u8) -> ::std::cmp::Ordering {
    (ayear * 12 + amonth as u16).cmp(&(byear * 12 + bmonth as u16))
}

fn calc_lang_stats_at_five_points(stats: &[Stat])  -> [LanguageStatSnapshot; 5]{
    let stats: Vec<&Stat> = stats.iter()
    .sorted_by(|&a, &b| a.time.cmp(&b.time))
    .collect();
    let first_time = dt_from_gittime(&stats.first().unwrap().time);
    let last_time = dt_from_gittime(&stats.last().unwrap().time);
    let difference = (last_time - first_time) / 4;
    let tmp = LanguageStatSnapshot::default();
    let mut got_stats = [calc_lang_stats(stats.first().unwrap()).unwrap(), tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone()];
    let mut counter: usize = 1;
    let mut iter = stats.iter().peekable();
    loop {
        let time = first_time + difference * (counter as i32);
        let stat = iter.next().unwrap();
        if let Some(next) = iter.peek() {
            if dt_from_gittime(&stat.time) < time && time < dt_from_gittime(&next.time) {
                got_stats[counter] = calc_lang_stats(next).unwrap();
                counter +=1;
            }
        } else {
            while counter < 5 {
                got_stats[counter] =calc_lang_stats(stat).unwrap();
                counter +=1;
            }
            break;
        }
    }
    for item in &got_stats {
        println!("{:?}", item);
    }
    got_stats
}

use std::ops::AddAssign;
fn calc_lang_stats(stat: &Stat) -> Result<LanguageStatSnapshot, Error>{
    let repo = Repository::open(".")?;
    let tree = repo.find_commit(stat.id)?.tree().unwrap();
    println!("In {}:", stat.id);
    fn map_blobs(tree: &Tree, repo: &Repository) -> Result<Vec<String>, Error> {
        let vecs: Vec<Vec<String>> = tree.iter()
        .map(|item| match item.kind().unwrap() {
            ObjectType::Blob => Ok(vec![item.name().unwrap().to_string()]),
            ObjectType::Tree => map_blobs(item.to_object(repo)?.as_tree().unwrap(), repo),
            _ => unreachable!()
        }).collect()?;
        Ok(vecs.into_iter().flat_map(|e| e).collect())
    }
    let mb = map_blobs(&tree, &repo)?;
    let exts = mb.iter().map(|s| s.rsplit('.').next().unwrap().to_string());
    let mut map = HashMap::new();
    for ext in exts {
        if !map.contains_key(&ext) {
            map.insert(ext.clone(), 0);
        }
        let s = map.get_mut(&ext).unwrap();
        s.add_assign(1);
    }
    let dt = dt_from_gittime(&stat.time);
    Ok(LanguageStatSnapshot(dt.year(), dt.month0(), map.into_iter().collect()))
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Clone)]
pub struct TopMonthContributer {
    pub year: u16,
    pub month: u8,
    pub sign: Author,
    pub stat: MainStat,
}

impl TopMonthContributer {
    fn new(year: u16, month: u8, author: Author, stat: MainStat) -> Self {
        TopMonthContributer {
            year: year,
            month: month,
            sign: author,
            stat: stat,
        }
    }
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct AllResultStat {
    pub top_monthly: Vec<TopMonthContributer>,
    pub common_stats: BTreeMap<String, ResultStat>,
    pub lang_stats: [LanguageStatSnapshot; 5],
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Default, Debug, Clone)]
pub struct LanguageStatSnapshot(i32, u32, Vec<(String, u32)>);

#[derive(RustcDecodable, RustcEncodable)]
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Author(pub String, pub String);

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Clone)]
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
