#![feature(question_mark)]
#![feature(stmt_expr_attributes)]
extern crate git2;
extern crate ansi_term;
extern crate chrono;
extern crate lazysort;
extern crate rustc_serialize;
extern crate num_cpus;
#[macro_use]
#[cfg(feature = "qt")]
extern crate qml;
extern crate time;
extern crate csv;
extern crate crossbeam;

use time::precise_time_s;

mod diff;
mod stats;
mod view;
mod csv_output;

use stats::*;
use diff::*;
use view::*;
use csv_output::*;

fn main() {
    let start = precise_time_s();
    let stats = if let Ok(s) = csv_load() {
        s
    } else {
        let s = gather_stats().unwrap();
        // Dump to start from cache later
        csv_dump(&s);
        s
    };
    let gather_time = precise_time_s() - start;

    let start = precise_time_s();
    let gathered = process(stats);
    let stat_time = precise_time_s() - start;

    println!("Gathered diffs data in {} secs and processed stats in {} secs",
             gather_time,
             stat_time);
    output(&gathered);
}
