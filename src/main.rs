mod config;
mod util;

use config::{Config, Type};
use iced::{Color, Element, widget};
use time::{Date, Duration, Month, Weekday};

#[derive(Default)]
struct App {
    month_selected: Option<Month>,
    offset_input: String,
    year_input: String,
    name_input: String,
    type_selected: Option<Type>,
    pay_input: String,
    configs: Vec<Config>,
}

#[derive(Debug, Clone)]
enum Message {
    MonthSelected(Month),
    OffsetInput(String),
    YearInput(String),
    NameInput(String),
    TypeSelected(Type),
    PayInput(String),
    PushPressed,
}

impl App {
    const CALENDAR_WIDTH: u16 = 120;
    const CALENDAR_HEIGHT: u16 = 100;
    const SPACING: u16 = 6;
    const CALENDAR_ROWS: u8 = 6;
    const CALENDAR_COLUMNS: u8 = util::WEEKDAYS.len() as u8;
    const CONFIG_WIDTH: u16 = 100;
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

    fn pay(&self) -> Option<u32> {
        self.pay_input.parse().ok()
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

    fn config(&self) -> Option<Config> {
        Some(Config {
            name: self.name_input.clone(),
            r#type: self.type_selected?,
            pay: self.pay()?,
        })
    }

    fn calendar_cell(&self, row: u8, column: u8) -> Element<Message> {
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

        util::rounded_container(widget::text(date_str).color(color))
            .width(Self::CALENDAR_WIDTH)
            .height(Self::CALENDAR_HEIGHT)
            .into()
    }

    fn view(&self) -> Element<Message> {
        use widget::{button, column, pick_list, row, scrollable, text, text_input};

        let year_month_offset = row![
            pick_list(util::MONTHS, self.month_selected, Message::MonthSelected),
            text_input("Offset", &self.offset_input).on_input(Message::OffsetInput),
            text_input("Year", &self.year_input).on_input(Message::YearInput),
        ]
        .spacing(Self::SPACING);

        let configs_input = row![
            text_input("Name", &self.name_input).on_input(Message::NameInput),
            pick_list(config::TYPES, self.type_selected, Message::TypeSelected),
            button("Push").on_press(Message::PushPressed),
        ]
        .spacing(Self::SPACING);

        let configs_top = row(["Name", "Pay"].map(|field| {
            util::rounded_container(text(field))
                .width(Self::CONFIG_WIDTH)
                .into()
        }))
        .spacing(Self::SPACING);

        let configs_body = column(self.configs.iter().map(|config| {
            row![
                util::rounded_container(text(&config.name)).width(Self::CONFIG_WIDTH),
                util::rounded_container(text(config.pay_to_string())).width(Self::CONFIG_WIDTH),
            ]
            .spacing(Self::SPACING)
            .into()
        }))
        .spacing(Self::SPACING);

        let calendar_top = row(util::WEEKDAYS.map(|weekday| {
            util::rounded_container(text(util::short_weekday(&weekday).to_string()))
                .width(Self::CALENDAR_WIDTH)
                .into()
        }))
        .spacing(Self::SPACING);

        let calendar_body = column((0..Self::CALENDAR_ROWS).map(|r| {
            row((0..Self::CALENDAR_COLUMNS).map(|c| self.calendar_cell(r, c)))
                .spacing(Self::SPACING)
                .into()
        }))
        .spacing(Self::SPACING);

        scrollable(
            column![
                util::bold_text("Date"),
                year_month_offset,
                util::bold_text("Configs"),
                configs_input,
                configs_top,
                configs_body,
                util::bold_text("Calendar"),
                calendar_top,
                calendar_body,
            ]
            .spacing(Self::SPACING)
            .padding(Self::SPACING),
        )
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::MonthSelected(month) => self.month_selected = Some(month),
            Message::OffsetInput(offset) => self.offset_input = offset,
            Message::YearInput(year) => self.year_input = year,
            Message::NameInput(name) => self.name_input = name,
            Message::TypeSelected(r#type) => self.type_selected = Some(r#type),
            Message::PayInput(pay) => self.pay_input = pay,
            Message::PushPressed => {
                if let Some(config) = self.config() {
                    self.configs.push(config)
                }
            }
        }
    }
}

fn main() -> iced::Result {
    iced::run("Calc", App::update, App::view)
}
