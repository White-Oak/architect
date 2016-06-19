#![cfg(feature = "html")]
use super::super::stats::*;

use std::collections::*;
use std::io::prelude::*;
use std::fs::File;
use rustc_serialize::json;
use std::process::Command;

pub fn output(gathered: &BTreeMap<String, ResultStat>) {
    let json_data = json::encode(&gathered).unwrap();
    let script = format!("var data = {};", json_data);
    let result: String = include_str!("template.html").replace("{architect-data}", &script);

    let mut f = File::create("result.html").unwrap();
    f.write_all(result.as_bytes()).unwrap();

    println!("Generated result.html, open it in your browser.");
    #[cfg(target_os = "linux")]
    Command::new("xdg-open")
        .arg("result.html")
        .output()
        .unwrap_or_else(|e| panic!("failed to open html automatically: {}", e));

    #[cfg(target_os = "macos")]
    Command::new("open")
        .arg("result.html")
        .output()
        .unwrap_or_else(|e| panic!("failed to open html automatically: {}", e));
}
