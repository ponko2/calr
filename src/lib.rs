use anyhow::{anyhow, ensure, Result};
use chrono::{Datelike, Local, NaiveDate};
use clap::Parser;
use std::str::FromStr;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, help = "Month name or number (1-12)", value_parser = parse_month)]
    month: Option<u32>,

    #[arg(short = 'y', long = "year", help = "Show whole current year", conflicts_with_all = ["month", "year"])]
    show_current_year: bool,

    #[arg(help = "Year (1-9999)", value_parser = parse_year)]
    year: Option<i32>,
}

#[derive(Debug)]
pub struct Config {
    month: Option<u32>,
    year: i32,
    today: NaiveDate,
}

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

pub fn get_args() -> Result<Config> {
    let args = Args::parse();
    let mut month = args.month;
    let mut year = args.year;
    let today = Local::now();
    if args.show_current_year {
        month = None;
        year = Some(today.year());
    } else if month.is_none() && year.is_none() {
        month = Some(today.month());
        year = Some(today.year());
    }
    Ok(Config {
        month,
        year: year.unwrap_or_else(|| today.year()),
        today: today.date_naive(),
    })
}

pub fn run(config: Config) -> Result<()> {
    dbg!(config);
    Ok(())
}

fn parse_int<T: FromStr>(val: &str) -> Result<T> {
    val.parse()
        .map_err(|_| anyhow!("Invalid integer \"{val}\""))
}

fn parse_year(year: &str) -> Result<i32> {
    parse_int(year).and_then(|num| {
        ensure!(
            (1..=9999).contains(&num),
            "year \"{year}\" not in the range 1 through 9999"
        );
        Ok(num)
    })
}

fn parse_month(month: &str) -> Result<u32> {
    match parse_int(month) {
        Ok(num) => {
            ensure!(
                (1..=12).contains(&num),
                "month \"{month}\" not in the range 1 through 12"
            );
            Ok(num)
        }
        _ => {
            let lower = &month.to_lowercase();
            MONTH_NAMES
                .iter()
                .position(|name| name.to_lowercase().starts_with(lower))
                .map(|i| i as u32 + 1)
                .ok_or_else(|| anyhow!("Invalid month \"{month}\""))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_int, parse_month, parse_year};

    #[test]
    fn test_parse_int() {
        // 正の整数をusizeとして解析する
        let res = parse_int::<usize>("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1usize);

        // 負の整数をi32として解析する
        let res = parse_int::<i32>("-1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), -1i32);

        // 数字以外の文字列を解析すると失敗する
        let res = parse_int::<i64>("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid integer \"foo\"");
    }

    #[test]
    fn test_parse_year() {
        let res = parse_year("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1i32);

        let res = parse_year("9999");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 9999i32);

        let res = parse_year("0");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "year \"0\" not in the range 1 through 9999"
        );

        let res = parse_year("10000");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "year \"10000\" not in the range 1 through 9999"
        );

        let res = parse_year("foo");
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_month() {
        let res = parse_month("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("12");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 12u32);

        let res = parse_month("jan");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("0");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "month \"0\" not in the range 1 through 12"
        );

        let res = parse_month("13");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "month \"13\" not in the range 1 through 12"
        );

        let res = parse_month("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid month \"foo\"");
    }
}
