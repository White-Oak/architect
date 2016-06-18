#![feature(question_mark)]
extern crate git2;
extern crate ansi_term;
extern crate chrono;
extern crate lazysort;
extern crate rustc_serialize;
extern crate num_cpus;
#[macro_use]
#[cfg(feature = "qt")]
extern crate qmlrs;
extern crate regex;

mod diff;
mod stats;
mod view;

use stats::*;
use diff::*;
use view::*;

use rustc_serialize::json;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let stats = gather_stats().unwrap();
    let gathered = process(stats);

    let encoded = json::encode(&gathered).unwrap();

    let mut f = File::create("out.json").unwrap();
    f.write_all(encoded.as_bytes()).unwrap();

    output(&gathered);
}
