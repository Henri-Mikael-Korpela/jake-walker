use std::io::{self, Write};
use std::{thread, time::Duration};

pub struct AnswerOption {
    text: &'static str,
    callback: fn(),
}
enum PrintColor {
    Green,
}
pub struct PrintSettings {
    visible: bool,
    delay: u64,
}

mod callbacks {
    use PrintSettings;

    pub fn normal(c: &char) -> PrintSettings {
        match c {
            '.' | ':' => PrintSettings {
                visible: true,
                delay: 350,
            },
            _ => PrintSettings {
                visible: true,
                delay: 20,
            },
        }
    }
    pub fn paragraph_change(_c: &char) -> PrintSettings {
        return PrintSettings {
            visible: true,
            delay: 75,
        };
    }
    pub fn question(_c: &char) -> PrintSettings {
        return PrintSettings {
            visible: true,
            delay: 100,
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
fn print_disable_color() {
    print!("\x1b[0m");
}
fn print_enable_color(color: PrintColor) {
    match color {
        PrintColor::Green => {
            print!("\x1b[0;32m");
        }
    }
}
fn print_question(question: &str, answers: &[AnswerOption]) {
    print_enable_color(PrintColor::Green);

    print(question, callbacks::question);
    print("\n\n", callbacks::paragraph_change);

    for i in 0..answers.len() {
        println!("{}. {}", i + 1, answers[i].text);
    }

    print_disable_color();

    println!();
}

mod actions {
    use callbacks;
    use print;
    use print_question;
    use AnswerOption;

    fn die_on_beach() {
        print("You stayed on the beach doing nothing. You thought someone would come to rescue you. But no one came.", callbacks::normal);
        print("\n\n", callbacks::paragraph_change);

        print("You died of starvation.\0", callbacks::normal);
        print("\n\n", callbacks::paragraph_change);

        fail_game();
    }
    fn fail_game() {
        print("GAME OVER\n", callbacks::normal);
    }
    pub fn start_story() {
        print("It is dark. You cannot see anything. You can only feel something soft touching your face. It is sand. Your eyes are opening. Now you see more clearly, but it is still dark.", callbacks::normal);
        print("\n\n", callbacks::paragraph_change);

        print("You have survived. You can't remember what happened, but one thing is for certain: you are in a strange place. It is somewhat scary to lay on the sand in this strange place.", callbacks::normal);
        print("\n\n", callbacks::paragraph_change);

        print("You rise up to sit down. You look around. There is no one around. It seems you are alone - at least for now...", callbacks::normal);
        print("\n\n", callbacks::paragraph_change);

        print("You may not remember how you ended up here, but you sure remember your name. Your name is Jake Walker. You are a scientiest. You have done research in your life, but maybe what you are about to face on this island is something extraordinary.", callbacks::normal);
        print("\n\n", callbacks::paragraph_change);

        print("Wait, an island? That's right, that is what you can see. You are on a coast, a vast ocean is before your very eyes. It is getting cold. You need a place to stay.", callbacks::normal);
        print("\n\n", callbacks::paragraph_change);

        let what_to_do_answers = [
            AnswerOption {
                text: "Stand up and start walking.",
                callback: start_walking,
            },
            AnswerOption {
                text: "Continue sitting on the beach.",
                callback: die_on_beach,
            },
        ];
        print_question("What do you do?", &what_to_do_answers);
    }
    fn start_walking() {}
}

fn main() {
    print("JAKE WALKER", callbacks::title);
    delay(500);
    print("\n\n", callbacks::paragraph_change);

    actions::start_story();
}
