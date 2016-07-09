use super::stats::AllResultStat;

mod cli;
mod qt;

pub fn output(gathered: &AllResultStat) {
    #[cfg(all(feature = "cli", not(feature = "qt")))]
    cli::output(gathered);

    #[cfg(feature = "qt")]
    qt::output(gathered);
}
