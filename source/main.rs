mod answer;
mod print;
use print::{print_question, print_text};
mod print_callbacks;
mod time;

fn get_answer_by_user_input(answers: &[answer::AnswerOption]) -> Option<&answer::AnswerOption> {
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
fn handle_answer(answers: &[answer::AnswerOption]) {
    use std::io::{self, Write};

    loop {
        match get_answer_by_user_input(answers) {
            Some(a) => {
                print_text("\n\n", print_callbacks::paragraph_change);
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

mod actions {
    use answer::AnswerOption;
    use handle_answer;
    use print_callbacks::{normal, paragraph_change, title};
    use print_question;
    use print_text;
    use time;

    fn die_on_beach() {
        print_text("You stayed on the beach doing nothing. You thought someone would come to rescue you. But no one came.", normal);
        print_text("\n\n", paragraph_change);

        print_text("You died of starvation.", normal);
        print_text("\n\n", paragraph_change);

        fail_game();
    }
    fn fail_game() {
        print_text("GAME OVER\n", normal);
    }
    fn go_alongside_river() {
        print_text("Unsure of what these dark objects were, Jake decided not to approach them. He saw a river nearby and decided to walk alongside it.", normal);
        print_text("\n\n", paragraph_change);
    }
    fn go_to_remains_of_plane() {
        print_text("Intrigued, Jake took more steps towards it. He thought he would find something useful there. He saw remains of a plane.", normal);
        print_text("\n\n", paragraph_change);
    }
    pub fn start_story() {
        print_text("JAKE WALKER", title);
        time::delay(500);
        print_text("\n\n", paragraph_change);

        print_text("It is dark. You cannot see anything. You can only feel something soft touching your face. It is sand. Your eyes are opening. Now you see more clearly, but it is still dark.", normal);
        print_text("\n\n", paragraph_change);

        print_text("You have survived. You can't remember what happened, but one thing is for certain: you are in a strange place. It is somewhat scary to lay on the sand in this strange place.", normal);
        print_text("\n\n", paragraph_change);

        print_text("You rise up to sit down. You look around. There is no one around. It seems you are alone - at least for now...", normal);
        print_text("\n\n", paragraph_change);

        print_text("You may not remember how you ended up here, but you sure remember your name. Your name is Jake Walker. You are a scientist. You have done research in your life, but maybe what you are about to face on this island is something extraordinary.", normal);
        print_text("\n\n", paragraph_change);

        print_text("Wait, an island? That's right, that is what you can see. You are on a coast, a vast ocean is before your very eyes. It is getting cold. You need a place to stay.", normal);
        print_text("\n\n", paragraph_change);

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
        print_text("Jake started to walk along the coastline. Jake could feel the cold in his hands. As he was walking, he noticed how cloudy it was in the darkness. Jake wanted to find any sign of civilization: maybe fire, maybe light. There was nothing.", normal);
        print_text("\n\n", paragraph_change);

        print_text(
            "A couple of minutes later Jake could see something. He saw some darks objects.",
            normal,
        );
        print_text("\n\n", paragraph_change);

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
    actions::start_story();
}
