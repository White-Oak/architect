#![cfg(feature = "html")]
use super::super::stats::*;

extern crate rustc_serialize;

use std::collections::*;
use std::io::prelude::*;
use std::fs::File;
use rustc_serialize::json;

pub fn output(gathered: &BTreeMap<String, ResultStat>) {
    let mut f = File::open("html/template.html").unwrap();
    let mut html = String::new();
    f.read_to_string(&mut html).unwrap();

    let json_data = json::encode(&gathered).unwrap();

    let script = format!("var data = {};", json_data);

    let result: String = html.replace("{architect-data}", &script);

    let mut f = File::create("html/result.html").unwrap();
    f.write_all(result.as_bytes()).unwrap();
}
