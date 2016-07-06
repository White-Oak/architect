#![cfg(feature = "qt")]
use super::super::stats::*;

use qml::*;

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
    let mut qalm = QListModel::new(&["date", "user", "commits", "adds", "dels"]);
    for t in &gathered.top_monthly {
        let vec: Vec<QVariant> = vec![format!("{}, {}", t.year, MONTHES[t.month as usize]).into(),
                                      (&t.sign.0 as &str).into(),
                                      (t.stat.commits as i32).into(),
                                      (t.stat.inserts as i32).into(),
                                      (t.stat.dels as i32).into()];
        qalm.insert_row(vec.into_iter());
    }
    let qvar = qalm.get_qvar();
    engine.set_property("contrs", &qvar);



    let mut languages: Vec<String> = Vec::new();
    let mut categories: Vec<String> = Vec::new();
    // Fill with all languages
    for &LanguageStatSnapshot(year, month, ref map) in gathered.lang_stats.iter() {
        for lang in map.keys() {
            if !languages.contains(&lang) {
                languages.push(lang.clone());
            }
        }
        let mut formed = format!("{}, {}", year, MONTHES[month as usize]);
        if categories.contains(&formed) {
            let mut start = 1;
            while categories.contains(&(formed.clone() + &format!(" ({})", start))) {
                start += 1;
            }
            formed = formed + &format!(" ({})", start);
        }
        categories.push(formed);
    }
    let categories: Vec<QVariant> = categories.into_iter().map(|s| s.into()).collect();

    let mut bars: Vec<QVariant> = Vec::new();
    for lang in languages.into_iter() {
        let mut lang_data: Vec<QVariant> = Vec::new();
        for &LanguageStatSnapshot(_, _, ref map) in &gathered.lang_stats {
            lang_data.push(if let Some(n) = map.get(&lang) {
                    *n as i32
                } else {
                    0
                }
                .into());
        }
        bars.push(qvarlist![lang, lang_data].into());
    }

    engine.set_and_store_property("values", qvarlist![categories, bars]);
    engine.load_data(include_str!("chart.qml"));

    engine.exec();
}
