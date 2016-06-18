use diff::*;

use std::io::prelude::*;
use std::fs::File;
use csv;

pub fn csv_dump(stats: &[Stat]) {
    let mut w = csv::Writer::from_memory();

    for record in stats {
        w.encode(LocalStat::from_stat(record.clone())).unwrap();
    }
    let mut f = File::create("data.csv").unwrap();
    f.write_all(w.as_bytes()).unwrap();
}

#[derive(RustcEncodable)]
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

impl LocalStat{
    fn from_stat(stat: Stat) -> Self {
        let message = match stat.message {
            Some(m) => m,
            None => "".to_string()
        };
        let message = message.replace("\n", "\\n");
        LocalStat{
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
