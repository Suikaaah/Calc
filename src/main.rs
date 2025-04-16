use iced::Element;
use time::{Month, Weekday};

#[derive(Default)]
struct App {
    month_selected: Option<Month>,
    offset_input: String,
    year_input: String,
}

#[derive(Debug, Clone)]
enum Message {
    MonthSelected(Month),
    OffsetInput(String),
    YearInput(String),
    UpdatePressed,
}

static MONTH_ARRAY: [Month; 12] = [
    Month::January,
    Month::February,
    Month::March,
    Month::April,
    Month::May,
    Month::June,
    Month::July,
    Month::August,
    Month::September,
    Month::October,
    Month::November,
    Month::December,
];

static WEEKDAY_ARRAY: [Weekday; 7] = [
    Weekday::Sunday,
    Weekday::Monday,
    Weekday::Tuesday,
    Weekday::Wednesday,
    Weekday::Thursday,
    Weekday::Friday,
    Weekday::Saturday,
];

fn short_weekday(weekday: &Weekday) -> &str {
    match weekday {
        Weekday::Sunday => "Sun",
        Weekday::Monday => "Mon",
        Weekday::Tuesday => "Tue",
        Weekday::Wednesday => "Wed",
        Weekday::Thursday => "Thu",
        Weekday::Friday => "Fri",
        Weekday::Saturday => "Sat",
    }
}

static CALENDAR_WIDTH: u16 = 120;
static CALENDAR_HEIGHT: u16 = 160;
static SPACING: u16 = 8;

impl App {
    fn view(&self) -> Element<Message> {
        use iced::widget::{button, column, container, pick_list, row, text, text_input};

        let year_month_offset = row![
            pick_list(MONTH_ARRAY, self.month_selected, Message::MonthSelected),
            text_input("Offset", &self.offset_input).on_input(Message::OffsetInput),
            text_input("Year", &self.year_input).on_input(Message::YearInput),
            button("Update").on_press(Message::UpdatePressed),
        ]
        .spacing(SPACING);

        let calendar_top = row(WEEKDAY_ARRAY.map(|weekday| {
            container(text(short_weekday(&weekday).to_string()))
                .width(CALENDAR_WIDTH)
                .style(container::rounded_box)
                .into()
        }))
        .spacing(SPACING);

        let calendar_body = row(WEEKDAY_ARRAY.map(|weekday| {
            container(text(short_weekday(&weekday).to_string()))
                .width(CALENDAR_WIDTH)
                .height(CALENDAR_HEIGHT)
                .style(container::rounded_box)
                .into()
        }))
        .spacing(SPACING);

        column![year_month_offset, calendar_top, calendar_body]
            .spacing(SPACING)
            .padding(SPACING)
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::MonthSelected(month) => self.month_selected = Some(month),
            Message::OffsetInput(offset) => self.offset_input = offset,
            Message::YearInput(year) => self.year_input = year,
            Message::UpdatePressed => {}
        }
    }
}

fn main() -> iced::Result {
    iced::run("Calc", App::update, App::view)
}
