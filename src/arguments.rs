use std::ffi::OsString;
use std::path::PathBuf;
use std::result::Result;
use clap::Parser;


#[derive(Parser, Debug)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
pub struct Args {
    /// Path to the config file
    #[clap(short = 'f', long, default_value = "config.yml")]
    pub file_path: PathBuf,

    /// Override default time in seconds to check websites
    #[clap(short = 't', long)]
    pub check_time_s: Option<u64>,
}
impl Args {
    pub fn try_parse<I, T>(iter: I) -> Result<Self, clap::Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let args = Args::try_parse_from(iter)?;
        Ok(args)
    }
}


#[cfg(test)]
mod tests;
