use std::io::{self, Write};
use std::{thread, time::Duration};

mod print_callbacks;

pub struct AnswerOption {
    text: &'static str,
    callback: fn(),
}
enum PrintColor {
    Green,
}

fn delay(delay_in_milliseconds: u64) {
    thread::sleep(Duration::from_millis(delay_in_milliseconds));
}
fn get_answer_by_user_input(answers: &[AnswerOption]) -> Option<&AnswerOption> {
    use std::io::stdin;

    let mut s = String::new();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    match s.parse::<usize>() {
        Ok(answer_number) => {
            if answer_number >= 1 && answer_number <= answers.len() {
                Some(&answers[answer_number - 1])
            } else {
                None
            }
        }
        Err(_e) => None,
    }
}
fn handle_answer(answers: &[AnswerOption]) {
    loop {
        match get_answer_by_user_input(answers) {
            Some(a) => {
                print("\n\n", print_callbacks::paragraph_change);
                (a.callback)();
                break;
            }
            None => {
                print!("Invalid answer was given. Please try again: ");
                io::stdout().flush().unwrap();
            }
        }
    }
}
fn print(value: &str, callback: fn(c: &char) -> print_callbacks::Settings) {
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
        PrintColor::Green => print!("\x1b[0;32m"),
    }
}
fn print_question(question: &str, answers: &[AnswerOption]) {
    print_enable_color(PrintColor::Green);

    print(question, print_callbacks::question);
    print("\n\n", print_callbacks::paragraph_change);

    for i in 0..answers.len() {
        println!("{}. {}", i + 1, answers[i].text);
    }

    print_disable_color();

    println!();
}

mod actions {
    use handle_answer;
    use print;
    use print_callbacks;
    use print_question;
    use AnswerOption;

    fn die_on_beach() {
        print("You stayed on the beach doing nothing. You thought someone would come to rescue you. But no one came.", print_callbacks::normal);
        print("\n\n", print_callbacks::paragraph_change);

        print("You died of starvation.", print_callbacks::normal);
        print("\n\n", print_callbacks::paragraph_change);

        fail_game();
    }
    fn fail_game() {
        print("GAME OVER\n", print_callbacks::normal);
    }
    fn go_alongside_river() {
        print("Unsure of what these dark objects were, Jake decided not to approach them. He saw a river nearby and decided to walk alongside it.", print_callbacks::normal);
        print("\n\n", print_callbacks::paragraph_change);
    }
    fn go_to_remains_of_plane() {
        print("Intrigued, Jake took more steps towards it. He thought he would find something useful there. He saw remains of a plane.", print_callbacks::normal);
        print("\n\n", print_callbacks::paragraph_change);
    }
    pub fn start_story() {
        print("It is dark. You cannot see anything. You can only feel something soft touching your face. It is sand. Your eyes are opening. Now you see more clearly, but it is still dark.", print_callbacks::normal);
        print("\n\n", print_callbacks::paragraph_change);

        print("You have survived. You can't remember what happened, but one thing is for certain: you are in a strange place. It is somewhat scary to lay on the sand in this strange place.", print_callbacks::normal);
        print("\n\n", print_callbacks::paragraph_change);

        print("You rise up to sit down. You look around. There is no one around. It seems you are alone - at least for now...", print_callbacks::normal);
        print("\n\n", print_callbacks::paragraph_change);

        print("You may not remember how you ended up here, but you sure remember your name. Your name is Jake Walker. You are a scientist. You have done research in your life, but maybe what you are about to face on this island is something extraordinary.", print_callbacks::normal);
        print("\n\n", print_callbacks::paragraph_change);

        print("Wait, an island? That's right, that is what you can see. You are on a coast, a vast ocean is before your very eyes. It is getting cold. You need a place to stay.", print_callbacks::normal);
        print("\n\n", print_callbacks::paragraph_change);

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
        handle_answer(&what_to_do_answers);
    }
    fn start_walking() {
        print("Jake started to walk along the coastline. Jake could feel the cold in his hands. As he was walking, he noticed how cloudy it was in the darkness. Jake wanted to find any sign of civilization: maybe fire, maybe light. There was nothing.", print_callbacks::normal);
        print("\n\n", print_callbacks::paragraph_change);

        print(
            "A couple of minutes later Jake could see something. He saw some darks objects.",
            print_callbacks::normal,
        );
        print("\n\n", print_callbacks::paragraph_change);

        let what_to_do_answers = [
            AnswerOption {
                text: "Go to them.",
                callback: go_to_remains_of_plane,
            },
            AnswerOption {
                text: "Avoid them.",
                callback: go_alongside_river,
            },
        ];
        print_question("What do you do?", &what_to_do_answers);
        handle_answer(&what_to_do_answers);
    }
}

fn main() {
    print("JAKE WALKER", print_callbacks::title);
    delay(500);
    print("\n\n", print_callbacks::paragraph_change);

    actions::start_story();
}
