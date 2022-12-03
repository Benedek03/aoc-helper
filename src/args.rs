use chrono::{DateTime, Datelike, FixedOffset, TimeZone, Utc};
use clap::{Parser, Subcommand};
use std::ops::RangeInclusive;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = last_year(), value_parser = year_in_range)]
    pub year: i32,

    #[arg(short, long, default_value_t = last_day(), value_parser = day_in_range)]
    pub day: u32,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// downloads input
    Fetch,
    /// submits an answer TODO
    Submit,
    /// opens the latest puzzle in the default browser
    Open,
}

fn last_year() -> i32 {
    let now: DateTime<FixedOffset> = FixedOffset::east_opt(-5 * 3600)
        .unwrap()
        .from_utc_datetime(&Utc::now().naive_utc());
    let mut last = now.year();
    if now.month() < 12 {
        last -= 1;
    }
    return last;
}

fn year_in_range(s: &str) -> Result<i32, String> {
    let year_range: RangeInclusive<i32> = 2015..=last_year();
    let year: i32 = s
        .parse()
        .map_err(|_| format!("`{}` isn't a number", s))
        .unwrap();
    if year_range.contains(&year) {
        Ok(year)
    } else {
        Err(format!(
            "year not in range {}-{}",
            year_range.start(),
            year_range.end()
        ))
    }
}

fn last_day() -> u32 {
    let now: DateTime<FixedOffset> = FixedOffset::east_opt(-5 * 3600)
        .unwrap()
        .from_utc_datetime(&Utc::now().naive_utc());
    let last = now.day();
    if last > 25 || now.month() != 12 {
        return 25;
    }
    return last;
}

fn day_in_range(s: &str) -> Result<u32, String> {
    let day_range: RangeInclusive<u32> = 1..=25;
    let day: u32 = s
        .parse()
        .map_err(|_| format!("`{}` isn't a number", s))
        .unwrap();
    if day_range.contains(&day) {
        Ok(day)
    } else {
        Err(format!(
            "day not in range {}-{}",
            day_range.start(),
            day_range.end()
        ))
    }
}

pub fn doublecheck_day(cli: &Cli) {
    let ld = last_day();
    if cli.year == last_year() && cli.day > ld {
        println!("Not so fast bro, you only have to sleep {} time(s) untill this day.", cli.day -ld);
        std::process::exit(1);
    }
}
