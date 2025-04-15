use iced::Element;

#[derive(Clone, Debug)]
enum Message {
    Increment,
    Decrement,
    YearInput(String),
    YearSubmit,
}

#[derive(Default)]
struct App {
    number: i32,
    year_str: String,
}

impl App {
    fn view(&self) -> Element<Message> {
        use iced::widget::{button, column, text, text_input};

        column![
            text_input("Year", &self.year_str)
                .on_input(Message::YearInput)
                .on_submit(Message::YearSubmit),
            button("+").on_press(Message::Increment),
            text(self.number.to_string()),
            button("-").on_press(Message::Decrement),
        ]
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.number += 1,
            Message::Decrement => self.number -= 1,
            Message::YearInput(input) => self.year_str = input,
            Message::YearSubmit => println!("{}", self.year().map(|x| x + 1).unwrap_or(420)),
        }
    }

    fn year(&self) -> Option<i32> {
        self.year_str.parse().ok()
    }
}

fn main() -> iced::Result {
    iced::run("Calc", App::update, App::view)
}
