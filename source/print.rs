use answer;
use print_callbacks;
use time;

enum PrintColor {
    Green,
}

fn print_disable_color() {
    print!("\x1b[0m");
}
fn print_enable_color(color: PrintColor) {
    match color {
        PrintColor::Green => print!("\x1b[0;32m"),
    }
}
pub fn print_question(question: &str, answers: &[answer::AnswerOption]) {
    print_enable_color(PrintColor::Green);

    print_text(question, print_callbacks::question);
    print_text("\n\n", print_callbacks::paragraph_change);

    for i in 0..answers.len() {
        println!("{}. {}", i + 1, answers[i].text);
    }

    print_disable_color();

    println!();
}
pub fn print_text(value: &str, callback: fn(c: &char) -> print_callbacks::Settings) {
    use std::io::{self, Write};

    for (i, c) in value.chars().enumerate() {
        let settings = callback(&c);

        if settings.visible {
            print!("{}", c);
            io::stdout().flush().unwrap();
        }
        if i + 1 != value.chars().count() {
            time::delay(settings.delay);
        }
    }

    io::stdout().flush().unwrap();
}
