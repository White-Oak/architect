#![cfg(feature = "qt")]
use super::super::stats::*;

use qmlrs::*;
use std::io::prelude::*;
use std::fs::File;
use std::io::Error;

pub fn output(gathered: &AllResultStat) {
    save_data(gathered)
        .unwrap();
    let mut engine = Engine::new();

    engine.load_local_file("chart.qml");

    engine.exec();
}

fn save_data(gathered: &AllResultStat) -> Result<(), Error> {
    let mut f = File::create("chart.qml")?;
    let total = gathered.common_stats.get("TOTAL").unwrap();
    let days = total.days;
    let mut data = include_str!("chart.qml").to_string();

    // Functions that replaces code phrases in a given string
    fn replace(c: &str, num: u32, i: usize, data: &str) -> String {
        let rep: &str = &num.to_string();
        data.replace(&format!("{}{}", c, i), rep)
    }
    for (i, item) in days.iter().enumerate().map(|(i, e)| (i + 1, e)) {
        data = replace("c", item.commits, i, &data);
        data = replace("a", item.inserts, i, &data);
        data = replace("d", item.dels, i, &data);
    }

    let times = total.daytimes;
    for (i, item) in times.iter().enumerate().map(|(i, e)| (i + 1, e)) {
        data = replace("cdt", item.commits, i, &data);
    }

    const MONTHES: [&'static str; 12] = ["January",
                                         "February",
                                         "March",
                                         "April",
                                         "May",
                                         "June",
                                         "Jule",
                                         "August",
                                         "September",
                                         "October",
                                         "November",
                                         "December"];
    let s: String = gathered.top_monthly.clone().into_iter()
        .map(|t| {
            format!(r#"ListElement {{
    date: "Year {}, {}"
    user: "{}"
    commits: {}
    adds: {}
    dels: {}
}}
"#,
                    t.year,
                    MONTHES[t.month as usize],
                    t.sign.0,
                    t.stat.commits,
                    t.stat.inserts,
                    t.stat.dels)
        })
        .collect();

    data = data.replace("LISTS", &s);

    let mut languages: Vec<String> = Vec::new();
    let mut dates = String::new();
    // Fill with all languages
    for (i, &LanguageStatSnapshot(year, month, ref map)) in gathered.lang_stats.iter().enumerate() {
        for lang in map.keys() {
            if !languages.contains(&lang){
                languages.push(lang.clone());
            }
        }
        dates = dates + &format!("\"Year {}, {}: {}\",", year, MONTHES[month as usize], i);
    }
    dates.pop();
    // replacements
    data = data.replace("LANG_DATES", &dates);
    let mut bars = String::new();
    for lang in languages.iter() {
        let mut lang_data = String::new();
        for &LanguageStatSnapshot(_, _, ref map) in &gathered.lang_stats {
            lang_data = lang_data + &format!("{},", if let Some(n) = map.get(lang){
                *n
            } else {
                0
            });
        }
        lang_data.pop();
        bars = bars + &format!("BarSet {{ label: \"{}\"; values: [{}] }}\n", &lang, &lang_data);
    }

    data = data.replace("LANG_BARS", &bars);
    f.write_all(data.as_bytes())?;
    Ok(())
}
