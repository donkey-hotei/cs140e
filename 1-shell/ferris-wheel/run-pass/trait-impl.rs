// FIXME: Make me pass! Diff budget: 25 lines.
#[derive(Debug)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

use Duration::{Seconds, MilliSeconds, Minutes};

impl Duration {
    fn to_minutes(&self) -> u64 {
        match self {
            MilliSeconds(value) => *value / 60_000,
            Seconds(value)      => *value as u64 / 60,
            Minutes(value)      => *value as u64
        }
    }
}

impl PartialEq for Duration {
    fn eq(&self, other: &Duration) -> bool {
        self.to_minutes() == other.to_minutes()
    }
}

fn main() {
    assert_eq!(Seconds(120), Minutes(2));
    assert_eq!(Seconds(420), Minutes(7));
    assert_eq!(MilliSeconds(420000), Minutes(7));
    assert_eq!(MilliSeconds(43000), Seconds(43));
}
