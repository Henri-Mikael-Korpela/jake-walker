use std::{thread, time::Duration};

pub fn delay(delay_in_milliseconds: u64) {
    thread::sleep(Duration::from_millis(delay_in_milliseconds));
}
