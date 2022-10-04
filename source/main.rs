use std::io::{self, Write};
use std::{thread, time::Duration};

pub struct PrintSettings {
    visible: bool,
    delay: u64,
}

mod callbacks {
    use PrintSettings;

    pub fn paragraph_change(_c: &char) -> PrintSettings {
        return PrintSettings {
            visible: true,
            delay: 75,
        };
    }
    pub fn title(_c: &char) -> PrintSettings {
        return PrintSettings {
            visible: true,
            delay: 200,
        };
    }
}

fn delay(delay_in_milliseconds: u64) {
    thread::sleep(Duration::from_millis(delay_in_milliseconds));
}
fn print(value: &str, callback: fn(c: &char) -> PrintSettings) {
    for (i, c) in value.chars().enumerate() {
        let settings = callback(&c);

        if settings.visible {
            print!("{}", c);
            io::stdout().flush().unwrap();
        }
        if i + 1 != value.chars().count() {
            delay(settings.delay);
        }
    }

    io::stdout().flush().unwrap();
}

fn main() {
    print("JAKE WALKER", callbacks::title);
    delay(500);
    print("\n\n\0", callbacks::paragraph_change);
}
