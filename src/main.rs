mod config;
mod util;

use config::{Config, TimeRange, Type, TypeForPickList};
use iced::{Color, Element, Length, alignment, widget};
use time::{Date, Duration, Month, Time, Weekday};

#[derive(Default)]
struct App {
    month_selected: Option<Month>,
    offset_input: String,
    year_input: String,
    name_input: String,
    type_selected: Option<TypeForPickList>,
    pay_input: String,
    hour_begin_input: String,
    minute_begin_input: String,
    hour_end_input: String,
    minute_end_input: String,
    configs: Vec<Config>,
}

#[derive(Debug, Clone)]
enum Message {
    MonthSelected(Month),
    OffsetInput(String),
    YearInput(String),
    NameInput(String),
    TypeSelected(TypeForPickList),
    PayInput(String),
    PushPressed,
    RemovePressed(usize),
    HourBeginInput(String),
    MinuteBeginInput(String),
    HourEndInput(String),
    MinuteEndInput(String),
}

impl App {
    const CALENDAR_HEIGHT: u16 = 120;
    const OFFSET_WIDTH: u16 = 100;
    const YEAR_WIDTH: u16 = 100;
    const NAME_WIDTH: u16 = 160;
    const PAY_WIDTH: u16 = 160;
    const TIME_WIDTH: u16 = 60;
    const DURATION_WIDTH: u16 = 160;
    const CHECKBOX_SIZE: u16 = 28;
    const SPACING: u16 = 6;
    const PADDING: u16 = 16;
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
        match self.month_selected? {
            Month::December => {
                Date::from_calendar_date(self.year()? + 1, Month::January, self.offset()?).ok()
            }
            month => Date::from_calendar_date(self.year()?, month.next(), self.offset()?).ok(),
        }
    }

    fn is_highlighted(&self, date: &Date) -> bool {
        self.highlight_begin().map(|x| &x <= date).unwrap_or(false)
            && self.highlight_end().map(|x| date < &x).unwrap_or(false)
    }

    fn time_range(&self) -> Option<TimeRange> {
        let hour_begin = self.hour_begin_input.parse().ok()?;
        let minute_begin = self.minute_begin_input.parse().ok()?;
        let hour_end = self.hour_end_input.parse().ok()?;
        let minute_end = self.minute_end_input.parse().ok()?;

        let begin = Time::from_hms(hour_begin, minute_begin, 0).ok()?;
        let end = Time::from_hms(hour_end, minute_end, 0).ok()?;
        Some(TimeRange { begin, end })
    }

    fn config(&self) -> Option<Config> {
        let r#type = match self.type_selected.as_ref()? {
            TypeForPickList::PerTime => Type::PerTime,
            TypeForPickList::PerHour => Type::PerHour(self.time_range()?),
        };

        Some(Config {
            name: self.name_input.clone(),
            r#type,
            pay: self.pay()?,
        })
    }

    fn calendar_cell(&self, r: u8, c: u8) -> Element<Message> {
        use widget::{checkbox, row, text};

        let nth = r * Self::CALENDAR_COLUMNS + c;

        let date = self
            .first_sunday()
            .and_then(|x| x.checked_add(Duration::days(nth as i64)));

        let active = date.map(|x| self.is_highlighted(&x)).unwrap_or(false);

        let chkbox = if active {
            Some(checkbox("", false).size(Self::CHECKBOX_SIZE))
        } else {
            None
        };

        let color = if active { Self::WHITE } else { Self::GRAY };

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

        util::rounded_container(
            row![]
                .push_maybe(chkbox)
                .push(text(date_str).color(color).width(Length::Fill)),
        )
        .width(Length::Fill)
        .height(Self::CALENDAR_HEIGHT)
        .into()
    }

    fn view(&self) -> Element<Message> {
        use widget::{button, column, pick_list, row, scrollable, text, text_input};

        let month_offset_year = row![
            pick_list(util::MONTHS, self.month_selected, Message::MonthSelected),
            text_input("Offset", &self.offset_input)
                .width(Self::OFFSET_WIDTH)
                .on_input(Message::OffsetInput),
            text_input("Year", &self.year_input)
                .width(Self::YEAR_WIDTH)
                .on_input(Message::YearInput),
        ]
        .spacing(Self::SPACING);

        let duration_input = match self.type_selected {
            Some(TypeForPickList::PerHour) => Some(
                row![
                    text_input("", &self.hour_begin_input)
                        .width(Self::TIME_WIDTH)
                        .on_input(Message::HourBeginInput),
                    text(":"),
                    text_input("", &self.minute_begin_input)
                        .width(Self::TIME_WIDTH)
                        .on_input(Message::MinuteBeginInput),
                    text("  -  "),
                    text_input("", &self.hour_end_input)
                        .width(Self::TIME_WIDTH)
                        .on_input(Message::HourEndInput),
                    text(":"),
                    text_input("", &self.minute_end_input)
                        .width(Self::TIME_WIDTH)
                        .on_input(Message::MinuteEndInput),
                ]
                .spacing(Self::SPACING),
            ),
            _ => None,
        };

        let configs_input = row![
            text_input("Name", &self.name_input)
                .width(Self::NAME_WIDTH)
                .on_input(Message::NameInput),
            text_input("Pay", &self.pay_input)
                .width(Self::PAY_WIDTH)
                .on_input(Message::PayInput),
            pick_list(
                config::TYPES_FOR_PICK_LIST,
                self.type_selected.clone(),
                Message::TypeSelected
            ),
        ]
        .push_maybe(duration_input)
        .push(button("Push").on_press(Message::PushPressed))
        .spacing(Self::SPACING);

        let configs_top = row![
            util::rounded_container("Name").width(Self::NAME_WIDTH),
            util::rounded_container("Pay").width(Self::PAY_WIDTH),
            util::rounded_container("Duration").width(Self::DURATION_WIDTH),
        ]
        .spacing(Self::SPACING);

        let configs_body = column(self.configs.iter().enumerate().map(|(i, config)| {
            row![
                text(&config.name).width(Self::NAME_WIDTH),
                text(config.pay_to_string()).width(Self::PAY_WIDTH),
                text(config.r#type.time_range_to_string()).width(Self::DURATION_WIDTH),
                button("Remove").on_press(Message::RemovePressed(i)),
            ]
            .align_y(alignment::Vertical::Center)
            .spacing(Self::SPACING)
            .into()
        }))
        .spacing(Self::SPACING);

        let calendar_top = row(util::WEEKDAYS.map(|weekday| {
            util::rounded_container(text(util::short_weekday(&weekday).to_string()))
                .width(Length::Fill)
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
                month_offset_year,
                util::bold_text("Configs"),
                configs_input,
                configs_top,
                configs_body,
                util::bold_text("Calendar"),
                calendar_top,
                calendar_body,
            ]
            .spacing(Self::SPACING)
            .padding(Self::PADDING),
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
            Message::RemovePressed(i) => {
                if i < self.configs.len() {
                    self.configs.remove(i);
                }
            }
            Message::HourBeginInput(x) => self.hour_begin_input = x,
            Message::MinuteBeginInput(x) => self.minute_begin_input = x,
            Message::HourEndInput(x) => self.hour_end_input = x,
            Message::MinuteEndInput(x) => self.minute_end_input = x,
        }
    }
}

fn main() -> iced::Result {
    iced::run("Calc", App::update, App::view)
}
