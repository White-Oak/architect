use std::collections::*;

use super::stats::ResultStat;

mod cli;
mod qt;

#[cfg(all(feature = "cli", not(feature = "qt")))]
pub fn output(gathered: &BTreeMap<String, ResultStat>) {
    cli::output(gathered);
}

#[cfg(feature = "qt")]
pub fn output(gathered: &BTreeMap<String, ResultStat>) {
    qt::output(gathered);
}
