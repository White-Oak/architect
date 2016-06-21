use diff::*;

use std::io::prelude::*;
use std::fs::File;
use csv::{Writer, Reader, Result};
use git2::*;

pub fn csv_dump(stats: &[Stat]) {
    let mut w = Writer::from_memory();

    for record in stats {
        w.encode(LocalStat::from(record.clone())).unwrap();
    }
    let mut f = File::create("data.csv").unwrap();
    f.write_all(w.as_bytes()).unwrap();
}

pub fn csv_load() -> Result<Vec<Stat>> {
    let mut r = Reader::from_file("data.csv")?;
    let ls_vec: Vec<LocalStat>  = r.decode().collect()?;
    Ok(ls_vec.into_iter().map(|ls| ls.into()).collect())
}

#[derive(RustcEncodable, RustcDecodable)]
struct LocalStat {
    pub id: String,
    pub author: String,
    pub email: String,
    pub inserts: u32,
    pub dels: u32,
    pub time: i64,
    pub offset_minutes: i32,
    pub message: String,
}

impl From<LocalStat> for Stat {
    fn from(stat: LocalStat) -> Self {
        let message = if stat.message.is_empty() {
            None
        } else {
            Some(stat.message)
        };
        Stat {
            id: Oid::from_str(&stat.id).unwrap(),
            author: stat.author,
            email: stat.email,
            inserts: stat.inserts,
            dels: stat.dels,
            time: Time::new(stat.time, stat.offset_minutes),
            message: message,
        }
    }
}

impl From<Stat> for LocalStat {
    fn from(stat: Stat) -> Self {
        let message = match stat.message {
            Some(m) => m,
            None => "".to_string(),
        };
        // let message = message.replace("\n", "\\n");
        LocalStat {
            id: stat.id.to_string(),
            author: stat.author,
            email: stat.email,
            inserts: stat.inserts,
            dels: stat.dels,
            time: stat.time.seconds(),
            offset_minutes: stat.time.offset_minutes(),
            message: message,
        }
    }
}
