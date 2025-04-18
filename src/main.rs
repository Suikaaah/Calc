mod config;
mod util;

use config::Config;
use iced::{Color, Element};
use time::{Date, Duration, Month, Weekday};

#[derive(Default)]
struct App {
    month_selected: Option<Month>,
    offset_input: String,
    year_input: String,
    configs: Vec<Config>,
}

#[derive(Debug, Clone)]
enum Message {
    MonthSelected(Month),
    OffsetInput(String),
    YearInput(String),
}

impl App {
    const CALENDAR_WIDTH: u16 = 120;
    const CALENDAR_HEIGHT: u16 = 100;
    const SPACING: u16 = 6;
    const CALENDAR_ROWS: u8 = 6;
    const CALENDAR_COLUMNS: u8 = util::WEEKDAYS.len() as u8;
    const WHITE: Color = Color::from_rgb(1.0, 1.0, 1.0);
    const GRAY: Color = Color::from_rgb(0.7, 0.7, 0.7);

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

    fn calendar_cell(&self, row: u8, column: u8) -> Element<Message> {
        use iced::widget::{container, text};

        let nth = row * Self::CALENDAR_COLUMNS + column;

        let date = self
            .first_sunday()
            .and_then(|x| x.checked_add(Duration::days(nth as i64)));

        let color = if date.map(|x| self.is_highlighted(&x)).unwrap_or(false) {
            Self::WHITE
        } else {
            Self::GRAY
        };

        let show_month = date.map(|x| x.day() == 1 || nth == 0).unwrap_or(false);

        let date_str = date
            .map(|x| {
                if show_month {
                    format!("{} {}", util::short_month(&x.month()), x.day())
                } else {
                    x.day().to_string()
                }
            })
            .unwrap_or_else(|| "N/A".to_string());

        container(text(date_str).color(color))
            .width(Self::CALENDAR_WIDTH)
            .height(Self::CALENDAR_HEIGHT)
            .style(container::rounded_box)
            .into()
    }

    fn view(&self) -> Element<Message> {
        use iced::widget::{column, container, pick_list, row, text, text_input};

        let year_month_offset = row![
            pick_list(util::MONTHS, self.month_selected, Message::MonthSelected),
            text_input("Offset", &self.offset_input).on_input(Message::OffsetInput),
            text_input("Year", &self.year_input).on_input(Message::YearInput),
        ]
        .spacing(Self::SPACING);

        let configs_top = row(config::FIELDS.map(|field| {
            container(text(field.to_string()))
                .width(Self::CALENDAR_WIDTH)
                .style(container::rounded_box)
                .into()
        }))
        .spacing(Self::SPACING);

        let configs_body = column((0..self.configs.len()).map(|r| {
            row(config::FIELDS.map(|field| {

            }))
        }))
        .spacing(Self::SPACING);

        let calendar_top = row(util::WEEKDAYS.map(|weekday| {
            container(text(util::short_weekday(&weekday).to_string()))
                .width(Self::CALENDAR_WIDTH)
                .style(container::rounded_box)
                .into()
        }))
        .spacing(Self::SPACING);

        let calendar_body = column((0..Self::CALENDAR_ROWS).map(|r| {
            row((0..Self::CALENDAR_COLUMNS).map(|c| self.calendar_cell(r, c)))
                .spacing(Self::SPACING)
                .into()
        }))
        .spacing(Self::SPACING);

        column![year_month_offset, configs_top, calendar_top, calendar_body]
            .spacing(Self::SPACING)
            .padding(Self::SPACING)
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
