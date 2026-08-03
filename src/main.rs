use std::env;

use croniter_rs::{CronExpression, CronIterator, CronResult, DateTime};

fn main() -> CronResult<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: croniter-rs <expression> [base-time]");
        std::process::exit(2);
    }

    let expression = CronExpression::parse(&args[1])?;
    let base = if args.len() > 2 {
        parse_datetime(&args[2])?
    } else {
        DateTime::from_ymd_opt(2024, 1, 1)?
    };

    let iter = CronIterator::new(expression, base);
    println!("{}", iter.next_match()?);
    Ok(())
}

fn parse_datetime(input: &str) -> CronResult<DateTime> {
    let parts: Vec<&str> = input.split(['-', ' ', ':']).collect();
    if parts.len() != 6 {
        return Err(croniter_rs::CronError::InvalidExpression(format!(
            "invalid datetime: {input}"
        )));
    }
    let year = parts[0].parse::<i32>().map_err(|_| croniter_rs::CronError::InvalidExpression(format!("invalid datetime: {input}")))?;
    let month = parts[1].parse::<u32>().map_err(|_| croniter_rs::CronError::InvalidExpression(format!("invalid datetime: {input}")))?;
    let day = parts[2].parse::<u32>().map_err(|_| croniter_rs::CronError::InvalidExpression(format!("invalid datetime: {input}")))?;
    let hour = parts[3].parse::<u32>().map_err(|_| croniter_rs::CronError::InvalidExpression(format!("invalid datetime: {input}")))?;
    let minute = parts[4].parse::<u32>().map_err(|_| croniter_rs::CronError::InvalidExpression(format!("invalid datetime: {input}")))?;
    let second = parts[5].parse::<u32>().map_err(|_| croniter_rs::CronError::InvalidExpression(format!("invalid datetime: {input}")))?;
    DateTime::new(year, month, day, hour, minute, second)
}
