use croniter_rs::{CronExpression, DateTime};

fn main() {
    let expr = CronExpression::parse("*/5 * * * *").unwrap();
    let base = DateTime::new(2024, 1, 1, 0, 0, 0).unwrap();
    let _ = expr.matches(&base);
}
