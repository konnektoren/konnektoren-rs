use chrono::{DateTime, Duration, Utc};

pub trait Timed {
    fn start(&mut self);
    fn update_end_time(&mut self);
    fn elapsed_time(&self) -> Option<Duration>;
    fn start_time(&self) -> Option<DateTime<Utc>>;
    fn end_time(&self) -> Option<DateTime<Utc>>;
}
