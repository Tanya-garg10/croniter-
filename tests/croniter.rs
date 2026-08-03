use croniter_rs::{CronExpression, CronIterator, CronResult, DateTime};

fn dt(y: i32, m: u32, d: u32, h: u32, min: u32, sec: u32) -> DateTime {
    DateTime::new(y, m, d, h, min, sec).unwrap()
}

#[test]
fn parses_and_matches_basic_schedule() -> CronResult<()> {
    let expr = CronExpression::parse("*/5 * * * *")?;
    assert!(expr.matches(&dt(2024, 1, 25, 4, 45, 0))?);
    assert!(!expr.matches(&dt(2024, 1, 25, 4, 46, 0))?);
    Ok(())
}

#[test]
fn finds_next_matches_after_base() -> CronResult<()> {
    let expr = CronExpression::parse("0 0 * * *")?;
    let iter = CronIterator::new(expr, dt(2024, 1, 1, 0, 0, 0));
    assert_eq!(iter.next_match()?, dt(2024, 1, 2, 0, 0, 0));
    Ok(())
}

#[test]
fn supports_seconds_and_day_or_semantics() -> CronResult<()> {
    let expr = CronExpression::parse("* * * * * *")?;
    assert!(expr.matches(&dt(2024, 1, 1, 0, 0, 1))?);

    let expr = CronExpression::parse("0 0 1 * wed")?;
    let iter = CronIterator::new(expr, dt(2024, 1, 25, 0, 0, 0));
    assert_eq!(iter.next_match()?, dt(2024, 1, 28, 0, 0, 0));
    Ok(())
}
