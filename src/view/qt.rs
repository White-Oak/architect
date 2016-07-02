#![cfg(feature = "qt")]
use super::super::stats::*;

use qml::*;
use std::io::prelude::*;
use std::fs::File;
use std::io::Error;

pub fn output(gathered: &AllResultStat) {
    let mut engine = QmlEngine::new();
    save_data(gathered, &mut engine);

}

fn save_data(gathered: &AllResultStat, engine: &mut QmlEngine) {
    let total = gathered.common_stats.get("TOTAL").unwrap();
    let days = total.days;

    // Functions that replaces code phrases in a given string
    fn replace(c: &str, num: u32, i: usize, engine: &mut QmlEngine) {
        let rep: &str = &num.to_string();
        engine.set_and_store_property(&format!("{}{}", c, i), rep)
    }
    for (i, item) in days.iter().enumerate().map(|(i, e)| (i + 1, e)) {
        replace("c", item.commits, i, engine);
        replace("a", item.inserts, i, engine);
        replace("d", item.dels, i, engine);
    }

    let times = total.daytimes;
    for (i, item) in times.iter().enumerate().map(|(i, e)| (i + 1, e)) {
        replace("cdt", item.commits, i, engine);
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
    let mut qalm = QListModel::new(&["date, user, commits, adds, dels"]);
    for t in &gathered.top_monthly {
        let vec: Vec<QVariant> = vec![(t.year as i32).into(),
                                      MONTHES[t.month as usize].into(),
                                      (&t.sign.0 as &str).into(),
                                      (t.stat.commits as i32).into(),
                                      (t.stat.inserts as i32).into(),
                                      (t.stat.dels as i32).into()];
        qalm.insert_row(vec.into_iter());
    }
    let qvar = qalm.get_qvar();
    engine.set_property("contrs", &qvar);

    // let mut languages: Vec<String> = Vec::new();
    // let mut dates = String::new();
    // // Fill with all languages
    // for (i, &LanguageStatSnapshot(year, month, ref map)) in gathered.lang_stats.iter().enumerate() {
    //     for lang in map.keys() {
    //         if !languages.contains(&lang) {
    //             languages.push(lang.clone());
    //         }
    //     }
    //     dates = dates + &format!("\"Year {}, {}: {}\",", year, MONTHES[month as usize], i);
    // }
    // dates.pop();
    // replacements
    // data = data.replace("LANG_DATES", &dates);
    // let mut bars = String::new();
    // for lang in languages.iter() {
    //     let mut lang_data = String::new();
    //     for &LanguageStatSnapshot(_, _, ref map) in &gathered.lang_stats {
    //         lang_data = lang_data +
    //                     &format!("{},",
    //                              if let Some(n) = map.get(lang) {
    //                                  *n
    //                              } else {
    //                                  0
    //                              });
    //     }
    //     lang_data.pop();
    //     bars = bars +
    //            &format!("BarSet {{ label: \"{}\"; values: [{}] }}\n",
    //                     &lang,
    //                     &lang_data);
    // }
    //
    // data = data.replace("LANG_BARS", &bars);
    // f.write_all(data.as_bytes())?;
    // Ok(())
    engine.load_data(include_str!("chart.qml"));

    engine.exec();
}
