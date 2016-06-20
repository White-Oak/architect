#![feature(question_mark)]
#![feature(stmt_expr_attributes)]
#![feature(iter_arith)]
extern crate git2;
extern crate ansi_term;
extern crate chrono;
extern crate lazysort;
extern crate rustc_serialize;
extern crate num_cpus;
#[macro_use]
#[cfg(feature = "qt")]
extern crate qmlrs;
extern crate time;
#[cfg(feature = "csvdump")]
extern crate csv;

use time::precise_time_s;

mod diff;
mod stats;
mod view;
mod csv_output;

use stats::*;
use diff::*;
use view::*;
#[cfg(feature = "csvdump")]
use csv_output::*;

fn main() {
    let start = precise_time_s();
    let stats = gather_stats().unwrap();
    let gather_time = precise_time_s() - start;

    #[cfg(feature = "csvdump")]
    csv_dump(&stats);

    let start = precise_time_s();
    let gathered = process(stats);
    let stat_time = precise_time_s() - start;

    println!("Gathered diffs data in {} secs and processed stats in {} secs",
             gather_time,
             stat_time);
    output(&gathered);
}
