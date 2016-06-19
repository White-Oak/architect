use super::stats::AllResultStat;

mod cli;
mod qt;
mod html;

pub fn output(gathered: &AllResultStat) {
    #[cfg(all(feature = "cli", not(feature = "qt"), not(feature = "html")))]
    cli::output(gathered);

    #[cfg(feature = "qt")]
    qt::output(gathered);

    #[cfg(feature = "html")]
    html::output(gathered);
}
