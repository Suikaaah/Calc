use iced::{Color, Element};
use time::{Date, Duration, Month, Weekday};

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
}

const fn short_weekday(weekday: &Weekday) -> &str {
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

const fn short_month(month: &Month) -> &str {
    match month {
        Month::January => "Jan",
        Month::February => "Feb",
        Month::March => "Mar",
        Month::April => "Apr",
        Month::May => "May",
        Month::June => "Jun",
        Month::July => "Jul",
        Month::August => "Aug",
        Month::September => "Sep",
        Month::October => "Oct",
        Month::November => "Nov",
        Month::December => "Dec",
    }
}

const CALENDAR_WIDTH: u16 = 120;
const CALENDAR_HEIGHT: u16 = 100;
const SPACING: u16 = 6;
const CALENDAR_ROWS: u8 = 6;
const CALENDAR_COLUMNS: u8 = 7;
const WHITE: Color = Color::from_rgb(1.0, 1.0, 1.0);
const GRAY: Color = Color::from_rgb(0.7, 0.7, 0.7);

impl App {
    fn year(&self) -> Option<i32> {
        self.year_input.parse().ok()
    }

    fn offset(&self) -> Option<u8> {
        self.offset_input.parse().ok()
    }

    fn date(&self) -> Option<Date> {
        Date::from_calendar_date(self.year()?, self.month_selected?, self.offset()?).ok()
    }

    fn first_sunday(&self) -> Option<Date> {
        self.date().map(|x| match x.weekday() {
            Weekday::Sunday => x,
            _ => x.prev_occurrence(Weekday::Sunday),
        })
    }

    fn highlight_begin(&self) -> Option<Date> {
        self.date()
    }

    fn highlight_end(&self) -> Option<Date> {
        Date::from_calendar_date(self.year()?, self.month_selected?.next(), self.offset()?).ok()
    }

    fn is_highlighted(&self, date: &Date) -> bool {
        self.highlight_begin().map(|x| &x <= date).unwrap_or(false)
            && self.highlight_end().map(|x| date < &x).unwrap_or(false)
    }

    fn view(&self) -> Element<Message> {
        use iced::widget::{column, container, pick_list, row, text, text_input};

        const MONTHS: [Month; 12] = [
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

        const WEEKDAYS: [Weekday; 7] = [
            Weekday::Sunday,
            Weekday::Monday,
            Weekday::Tuesday,
            Weekday::Wednesday,
            Weekday::Thursday,
            Weekday::Friday,
            Weekday::Saturday,
        ];

        let year_month_offset = row![
            pick_list(MONTHS, self.month_selected, Message::MonthSelected),
            text_input("Offset", &self.offset_input).on_input(Message::OffsetInput),
            text_input("Year", &self.year_input).on_input(Message::YearInput),
        ]
        .spacing(SPACING);

        let calendar_top = row(WEEKDAYS.map(|weekday| {
            container(text(short_weekday(&weekday).to_string()))
                .width(CALENDAR_WIDTH)
                .style(container::rounded_box)
                .into()
        }))
        .spacing(SPACING);

        let calendar_body = column((0..CALENDAR_ROWS).map(|r| {
            row((0..CALENDAR_COLUMNS).map(|c| {
                let nth = r * CALENDAR_COLUMNS + c;

                let date = self
                    .first_sunday()
                    .and_then(|x| x.checked_add(Duration::days(nth as i64)));

                let color = if date.map(|x| self.is_highlighted(&x)).unwrap_or(false) {
                    WHITE
                } else {
                    GRAY
                };

                let show_month = date.map(|x| x.day() == 1 || nth == 0).unwrap_or(false);

                let date_str = date
                    .map(|x| {
                        if show_month {
                            format!("{} {}", short_month(&x.month()), x.day())
                        } else {
                            x.day().to_string()
                        }
                    })
                    .unwrap_or_else(|| "N/A".to_string());

                container(text(date_str).color(color))
                    .width(CALENDAR_WIDTH)
                    .height(CALENDAR_HEIGHT)
                    .style(container::rounded_box)
                    .into()
            }))
            .spacing(SPACING)
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
        }
    }
}

fn main() -> iced::Result {
    iced::run("Calc", App::update, App::view)
}
