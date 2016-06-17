use stats::*;

use std::collections::*;
use qmlrs::*;
use std::io::prelude::*;
use std::fs::File;
use std::io::Error;
use regex::Regex;

pub fn view(gathered: &BTreeMap<String, ResultStat>) {
    save_data(gathered.get("TOTAL").unwrap()).unwrap();
    let mut engine = Engine::new();

    engine.load_local_file("chart.qml");

    engine.exec();
}

fn save_data(total: &ResultStat) -> Result<(), Error>{
    let mut f = File::create("chart.qml")?;
    let days = total.days;
    let mut data = include_str!("chart.qml").to_string();
    for (i, item) in days.iter().enumerate() {
        let i = i + 1;
        let re = Regex::new(&format!("c{}", i)).unwrap();
        let rep: &str = &item.commits.to_string();
        data = re.replace(&data, rep);
        let re = Regex::new(&format!("a{}", i)).unwrap();
        let rep: &str = &item.inserts.to_string();
        data = re.replace(&data, rep);
        let re = Regex::new(&format!("d{}", i)).unwrap();
        let rep: &str = &item.dels.to_string();
        data = re.replace(&data, rep);
    }

    let times = total.daytimes;
    for (i, item) in times.iter().enumerate() {
        let i = i + 1;
        let re = Regex::new(&format!("cd{}", i)).unwrap();
        let rep: &str = &item.commits.to_string();
        data = re.replace(&data, rep);
    }
    f.write_all(data.as_bytes())?;
    Ok(())
}
