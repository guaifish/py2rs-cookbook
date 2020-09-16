use chrono::prelude::*;

fn main() {
    let utc = Utc::now();
    let local = Local::now();
    println!("{:?}", utc);
    println!("{:?}", local);
    let today = Local::today();
    println!("{:?}", today.format("%Y-%m-%d").to_string());

    let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    assert_eq!(
        dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        "2014-11-28 12:00:09"
    );
    assert_eq!(dt.to_string(), "2014-11-28 12:00:09 UTC");

    let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    let fixed_dt = dt.with_timezone(&FixedOffset::east(9 * 3600));

    assert_eq!(
        "2014-11-28T12:00:09Z".parse::<DateTime<Utc>>(),
        Ok(dt.clone())
    );
    assert_eq!(
        "2014-11-28T21:00:09+09:00".parse::<DateTime<Utc>>(),
        Ok(dt.clone())
    );
    assert_eq!(
        "2014-11-28T21:00:09+09:00".parse::<DateTime<FixedOffset>>(),
        Ok(fixed_dt.clone())
    );
}
