pub struct Settings {
    pub visible: bool,
    pub delay: u64,
}

pub fn normal(c: &char) -> Settings {
    match c {
        '.' | ':' => Settings {
            visible: true,
            delay: 350,
        },
        _ => Settings {
            visible: true,
            delay: 20,
        },
    }
}
pub fn paragraph_change(_c: &char) -> Settings {
    Settings {
        visible: true,
        delay: 75,
    }
}
pub fn question(_c: &char) -> Settings {
    Settings {
        visible: true,
        delay: 100,
    }
}
pub fn title(_c: &char) -> Settings {
    Settings {
        visible: true,
        delay: 200,
    }
}
