use litime::minute::get_minute;

#[test]
/// Get a quote 100 times for every possible timestamps.
/// Each quote is retrieved 100 times to ensure (ish) that multiple random quotes for some
/// timestamps don't cause any panics.
fn test_get_minute_sfw() {
    for hour in 1..24 {
        for minute in 1..60 {
            let timestamp = format!("{:0width$}:{:0width$}", hour, minute, width = 2);
            for _ in 1..100 {
                get_minute(&timestamp, true).expect(&timestamp);
            }
        }
    }
}

#[test]
fn test_get_minute_nsfw() {
    for hour in 1..24 {
        for minute in 1..60 {
            let timestamp = format!("{:0width$}:{:0width$}", hour, minute, width = 2);
            for _ in 1..100 {
                get_minute(&timestamp, false).expect(&timestamp);
            }
        }
    }
}
