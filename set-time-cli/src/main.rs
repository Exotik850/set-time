use chrono::{DateTime, NaiveDateTime, Utc};
use clap::Parser;
use set_time::{SetTimeError, set_time};

const DATE_TIME_FMT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Parser)]
#[command(
    name = "set-time",
    about = "A cross-platform CLI tool to set the system time"
)]
pub struct CliArgs {
    #[arg(help = "The new system time in the format 'YYYY-MM-DD HH:MM:SS'")]
    pub time: String,
}

fn main() -> Result<(), SetTimeError> {
    let args = CliArgs::parse();
    let new_time: DateTime<Utc> = NaiveDateTime::parse_from_str(&args.time, DATE_TIME_FMT)
        .map_err(|_| SetTimeError::InvalidTime)?
        .and_utc();
    set_time(new_time)
}
