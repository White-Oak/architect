use std::collections::*;

use super::stats::ResultStat;

mod cli;
mod html;
mod qt;

#[cfg(all(feature = "cli", not(feature = "qt"), not(feature = "html")))]
pub fn output(gathered: &BTreeMap<String, ResultStat>) {
    cli::output(gathered);
}

#[cfg(feature = "qt")]
pub fn output(gathered: &BTreeMap<String, ResultStat>) {
    qt::output(gathered);
}

#[cfg(feature = "html")]
pub fn output(gathered: &BTreeMap<String, ResultStat>) {
    html::output(gathered);
}
