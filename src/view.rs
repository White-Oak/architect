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
    let mut data = r#"
import QtQuick 2.0
import QtCharts 2.0

import QtQuick.Controls 1.2
ApplicationWindow {

  visible: true
  title: "Architect View"

  ChartView {
      width: 800
      height: 500
      theme: ChartView.ChartThemeBrownSand
      antialiasing: true

      BarSeries {
          id: mySeries
          axisX: BarCategoryAxis { categories: ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun" ] }
          BarSet { label: "Additions"; values: [a1, a2, a3, a4, a5, a6, a7] }
          BarSet { label: "Deletions"; values: [d1, d2, d3, d4, d5, d6, d7] }
      }
  }
}
"#.to_string();
    for (i, item) in days.iter().enumerate() {
        let i = i + 1;
        let re = Regex::new(&format!("a{}", i)).unwrap();
        let rep: &str = &item.inserts.to_string();
        data = re.replace(&data, rep);
        let re = Regex::new(&format!("d{}", i)).unwrap();
        let rep: &str = &item.dels.to_string();
        data = re.replace(&data, rep);
    }
    f.write_all(data.as_bytes())?;
    Ok(())
}
