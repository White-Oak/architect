use std::collections::*;

use super::stats::ResultStat;

mod cli;
mod qt;
mod html;

pub fn output(gathered: &BTreeMap<String, ResultStat>) {
    #[cfg(all(feature = "cli", not(feature = "qt"), not(feature = "html")))]
    cli::output(gathered);

    #[cfg(feature = "qt")]
    qt::output(gathered);

    #[cfg(feature = "html")]
    html::output(gathered);
}
