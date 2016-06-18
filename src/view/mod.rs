use std::collections::*;

use super::stats::ResultStat;

mod cli;
mod qt;

pub fn output(gathered: &BTreeMap<String, ResultStat>) {
    #[cfg(all(feature = "cli", not(feature = "qt")))]
    cli::output(gathered);

    #[cfg(feature = "qt")]
    qt::output(gathered);
}
