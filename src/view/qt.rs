#![cfg(feature = "qt")]
use super::super::stats::*;

use std::collections::*;
use qmlrs::*;
use std::io::prelude::*;
use std::fs::File;
use std::io::Error;
use regex::Regex;

pub fn output(gathered: &BTreeMap<String, ResultStat>) {
    save_data(gathered.get("TOTAL").unwrap()).unwrap();
    let mut engine = Engine::new();

    engine.load_local_file("chart.qml");

    engine.exec();
}

fn save_data(total: &ResultStat) -> Result<(), Error> {
    let mut f = File::create("chart.qml")?;
    let days = total.days;
    let mut data = include_str!("chart.qml").to_string();

    // Functions that replaces code phrases in a given string
    fn replace(c: &str, num: u32, i: usize, data: &str) -> String {
        let re = Regex::new(&format!("{}{}", c, i)).unwrap();
        let rep: &str = &num.to_string();
        re.replace(&data, rep)
    }
    for (i, item) in days.iter().enumerate().map(|(i, e)| (i + 1, e)) {
        data = replace("c", item.commits, i, &data);
        data = replace("a", item.inserts, i, &data);
        data = replace("d", item.dels, i, &data);
    }

    let times = total.daytimes;
    for (i, item) in times.iter().enumerate().map(|(i, e)| (i + 1, e)) {
        data = replace("cd", item.commits, i, &data);
    }
    f.write_all(data.as_bytes())?;
    Ok(())
}
