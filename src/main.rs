#![windows_subsystem = "windows"]

mod cell;
mod config;
mod failure;
mod util;

use cell::Cell;
use config::{Config, HourMinute, Type, TypeForPickList};
use failure::Failure;
use iced::{Color, Element, Length, Size, Theme, alignment, theme, widget};
use std::{collections::BTreeMap, fs};
use time::{Date, Duration, Month, Weekday};
use util::Some;

struct App {
    month_selected: Option<Month>,
    offset_input: String,
    year_input: String,
    name_input: String,
    type_selected: Option<TypeForPickList>,
    pay_input: String,
    hour_input: String,
    minute_input: String,
    configs: BTreeMap<String, Config>,
    cells: [Cell; Self::CALENDAR_COLUMNS as usize * Self::CALENDAR_ROWS as usize],
    filename_input: String,
    filename_selected: Option<String>,
    title: String,
}

impl Default for App {
    fn default() -> Self {
        let current_date = util::current_date();

        Self {
            month_selected: match current_date {
                Some(x) => x.month().previous(),
                None => Month::January,
            }
            .some(),
            offset_input: Default::default(),
            year_input: current_date
                .map(|x| x.year().to_string())
                .unwrap_or_default(),
            name_input: Default::default(),
            type_selected: Some(TypeForPickList::PerHour),
            pay_input: Default::default(),
            hour_input: Default::default(),
            minute_input: Default::default(),
            configs: Default::default(),
            cells: std::array::from_fn(|_| Default::default()),
            filename_input: Default::default(),
            filename_selected: Default::default(),
            title: "Calc".to_string(),
        }
    }
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
    RemovePressed(String),
    RemoveFilePressed,
    HourInput(String),
    MinuteInput(String),
    AddPressed(String),
    CellChecked(bool, u8),
    CellButtonPressed(String, u8),
    DeselectPressed,
    FilenameInput(String),
    FilenameSelected(String),
    SavePressed,
    LoadPressed,
    WeekdayPressed(Weekday),
}

impl App {
    const OFFSET_WIDTH: u16 = 80;
    const YEAR_WIDTH: u16 = 80;
    const NAME_WIDTH: u16 = 162;
    const PAY_WIDTH: u16 = 130;
    const DURATION_WIDTH: u16 = 81;
    const COUNT_WIDTH: u16 = 36;
    const SUM_WIDTH: u16 = 122;
    const RIGHT_WIDTH: u16 = 620;
    const CHECKBOX_SIZE: u16 = 28;
    const RESULT_SIZE: u16 = 32;
    const SPACING: u16 = 6;
    const PADDING: u16 = 16;
    const CALENDER_VERTICAL_SPACING: u16 = Self::SPACING * 4;
    const CALENDAR_ROWS: u8 = 6;
    const CALENDAR_COLUMNS: u8 = util::WEEKDAYS.len() as u8;

    fn reset_title(&mut self) {
        self.title = "Calc".to_string();
    }

    fn set_title(&mut self, title: &str) {
        self.title = format!("Calc - {}", title)
    }

    fn set_failure(&mut self, failure: Failure) {
        self.set_title(failure.to_string().as_ref())
    }

    fn get_cell(&self, i: usize) -> Result<&Cell, Failure> {
        self.cells.get(i).ok_or(Failure::Cell(i))
    }

    fn get_cell_mut(&mut self, i: usize) -> Result<&mut Cell, Failure> {
        self.cells.get_mut(i).ok_or(Failure::Cell(i))
    }

    fn year(&self) -> Result<i32, Failure> {
        self.year_input.parse().map_err(|_| Failure::Year)
    }

    fn offset(&self) -> Result<u8, Failure> {
        self.offset_input.parse().map_err(|_| Failure::Offset)
    }

    fn month(&self) -> Month {
        self.month_selected
            .expect("unreachable because something is always selected")
    }

    fn r#type(&self) -> TypeForPickList {
        self.type_selected
            .expect("unreachable because something is always selected")
    }

    fn date(&self) -> Result<Date, Failure> {
        Date::from_calendar_date(self.year()?, self.month(), self.offset()?)
            .map_err(|_| Failure::Date)
    }

    fn pay(&self) -> Result<u32, Failure> {
        self.pay_input.parse().map_err(|_| Failure::Pay)
    }

    fn first_sunday(&self) -> Result<Date, Failure> {
        self.date().map(|x| match x.weekday() {
            Weekday::Sunday => x,
            _ => x.prev_occurrence(Weekday::Sunday),
        })
    }

    fn highlight_begin(&self) -> Result<Date, Failure> {
        self.date()
    }

    fn highlight_end(&self) -> Result<Date, Failure> {
        match self.month() {
            Month::December => {
                Date::from_calendar_date(self.year()? + 1, Month::January, self.offset()?)
            }
            month => Date::from_calendar_date(self.year()?, month.next(), self.offset()?),
        }
        .map_err(|_| Failure::Date)
    }

    fn is_highlighted(&self, date: &Date) -> bool {
        self.highlight_begin().map(|x| &x <= date).unwrap_or(false)
            && self.highlight_end().map(|x| date < &x).unwrap_or(false)
    }

    fn duration(&self) -> Result<HourMinute, Failure> {
        let parse_map = |input: &str| input.parse().map_err(|_| Failure::DurationParse);

        let hour = parse_map(&self.hour_input)?;
        let minute = parse_map(&self.minute_input)?;

        HourMinute::from_hm(hour, minute)
    }

    fn config(&self) -> Result<Config, Failure> {
        Ok(Config {
            pay: self.pay()?,
            r#type: match self.r#type() {
                TypeForPickList::PerTime => Type::PerTime,
                TypeForPickList::PerHour => Type::PerHour(self.duration()?),
            },
        })
    }

    fn clear_cells(&mut self) {
        for cell in &mut self.cells {
            cell.clear();
        }
    }

    fn clear_added(&mut self) {
        for cell in &mut self.cells {
            cell.clear_added();
        }
    }

    fn deselect(&mut self) {
        for cell in &mut self.cells {
            cell.deselect();
        }
    }

    fn load(&self) -> Result<BTreeMap<String, Config>, Failure> {
        let filename = self.filename_selected.as_ref().ok_or(Failure::Load)?;
        let read = fs::read_to_string(format!("{}.json", filename)).map_err(|_| Failure::Load)?;
        serde_json::from_str(&read).map_err(|_| Failure::Load)
    }

    fn save(&self) -> Result<(), Failure> {
        if self.filename_input.is_empty() {
            return Err(Failure::SaveEmptyName);
        }

        match serde_json::to_string(&self.configs) {
            Ok(to_write) => fs::write(format!("{}.json", self.filename_input), to_write)
                .map_err(|_| Failure::Save),
            Err(_) => Err(Failure::Save),
        }
    }

    fn cell_date(&self, i: usize) -> Result<Date, Failure> {
        self.first_sunday()
            .and_then(|x| x.checked_add(Duration::days(i as i64)).ok_or(Failure::Date))
    }

    fn find_jsons(&self) -> Result<Vec<String>, Failure> {
        let dir = fs::read_dir("./").map_err(|_| Failure::Load)?;

        let mut retval = Vec::new();

        for entry in dir {
            let path = entry.map_err(|_| Failure::Load)?.path();

            if let Some("json") = path.extension().and_then(|x| x.to_str()) {
                if let Some(filename) = path.file_stem().and_then(|x| x.to_str()) {
                    retval.push(filename.to_string())
                }
            }
        }

        Ok(retval)
    }

    fn remove_file(&self) -> Result<(), Failure> {
        let filename = self.filename_selected.as_ref().ok_or(Failure::FileRemove)?;
        fs::remove_file(format!("{filename}.json")).map_err(|_| Failure::FileRemove)
    }

    fn calendar_cell(&self, r: u8, c: u8) -> Element<Message> {
        use widget::{checkbox, column, row, text};

        let nth = r * Self::CALENDAR_COLUMNS + c;
        let cell = self
            .get_cell(nth as usize)
            .expect("supposed to be unreachable");
        let date = self.cell_date(nth as usize);
        let active = date.map(|x| self.is_highlighted(&x)).unwrap_or(false);

        let chkbox = {
            let base = checkbox("", cell.selected).size(Self::CHECKBOX_SIZE);

            if active {
                base.on_toggle(move |b| Message::CellChecked(b, nth))
            } else {
                base.style(checkbox::secondary)
            }
        };

        let show_month = date.map(|x| x.day() == 1 || nth == 0).unwrap_or(false);

        let date_str = date
            .map(|x| {
                if show_month {
                    format!("{} {}", util::short_month(x.month()), x.day())
                } else {
                    x.day().to_string()
                }
            })
            .unwrap_or_else(|_| "N/A".to_string());

        let date_text = text(date_str).width(Length::Fill).style(if active {
            text::base
        } else {
            text::secondary
        });

        column![
            row![chkbox, date_text],
            column(cell.config_names.iter().map(|name| {
                util::colored_button(
                    text(name.as_str())
                        .width(Length::Fill)
                        .align_x(alignment::Horizontal::Center),
                    util::get_color(name),
                )
                .padding(0)
                .on_press(Message::CellButtonPressed(name.to_owned(), nth))
                .into()
            }))
            .spacing(Self::SPACING)
        ]
        .spacing(Self::SPACING)
        .padding(Self::SPACING)
        .width(Length::Fill)
        .into()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn view(&self) -> Element<Message> {
        use widget::{Space, button, column, pick_list, row, scrollable, text, text_input};

        let space = || Space::new(Self::SPACING, Self::SPACING);

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
            Some(TypeForPickList::PerHour) => row![
                text_input("Hour", &self.hour_input).on_input(Message::HourInput),
                util::monospace_text(":"),
                text_input("Minute", &self.minute_input).on_input(Message::MinuteInput),
            ]
            .align_y(alignment::Vertical::Center)
            .spacing(Self::SPACING)
            .some(),
            _ => None,
        };

        let configs_io = row![
            text_input("Filename", &self.filename_input)
                .width(Length::Fill)
                .on_input(Message::FilenameInput),
            button("Save").on_press(Message::SavePressed),
            pick_list(
                self.find_jsons().unwrap_or_default(),
                self.filename_selected.as_ref(),
                Message::FilenameSelected
            ),
            button("Load").on_press(Message::LoadPressed),
            button("Delete").on_press(Message::RemoveFilePressed),
        ]
        .spacing(Self::SPACING);

        let configs_input = row![
            text_input("Name", &self.name_input)
                .width(Self::NAME_WIDTH)
                .on_input(Message::NameInput),
            text_input("Pay", &self.pay_input)
                .width(Self::PAY_WIDTH)
                .on_input(Message::PayInput),
            pick_list(
                config::TYPES_FOR_PICK_LIST,
                self.type_selected,
                Message::TypeSelected
            ),
        ]
        .push_maybe(duration_input)
        .push(button("v").on_press(Message::PushPressed))
        .spacing(Self::SPACING);

        let configs_top = if self.configs.is_empty() {
            None
        } else {
            let top = |name, width| {
                util::rounded_container(name)
                    .width(width)
                    .align_x(alignment::Horizontal::Center)
            };

            row![
                top("Name", Self::NAME_WIDTH),
                top("Pay", Self::PAY_WIDTH),
                top("Duration", Self::DURATION_WIDTH),
                top("#", Self::COUNT_WIDTH),
                top("Sum", Self::SUM_WIDTH),
            ]
            .spacing(Self::SPACING)
            .some()
        };

        let configs_input_and_top = column![configs_input]
            .push_maybe(configs_top)
            .spacing(Self::SPACING);

        let mut sum = 0;

        let configs_body = column(self.configs.iter().map(|(name, config)| {
            let count = self
                .cells
                .iter()
                .filter(|cell| cell.contains(name.as_str()))
                .count();

            sum += config.sum(count);

            row![
                util::colored_button(
                    text(name)
                        .width(Length::Fill)
                        .align_x(alignment::Horizontal::Center),
                    util::get_color(name),
                )
                .width(Self::NAME_WIDTH)
                .padding(0)
                .on_press(Message::AddPressed(name.to_owned())),
                util::monospace_text(config.pay_to_string())
                    .width(Self::PAY_WIDTH)
                    .align_x(alignment::Horizontal::Right),
                util::monospace_text(config.r#type.duration_to_string())
                    .width(Self::DURATION_WIDTH)
                    .align_x(alignment::Horizontal::Center),
                util::monospace_text(util::comma_separated(count as u32))
                    .width(Self::COUNT_WIDTH)
                    .align_x(alignment::Horizontal::Right),
                util::monospace_text(util::yen(config.sum(count)))
                    .width(Self::SUM_WIDTH)
                    .align_x(alignment::Horizontal::Right),
                button("x").on_press(Message::RemovePressed(name.to_owned())),
            ]
            .align_y(alignment::Vertical::Center)
            .spacing(Self::SPACING)
            .into()
        }))
        .spacing(Self::SPACING);

        let result_body = util::monospace_text(util::yen(sum)).size(Self::RESULT_SIZE);

        let calendar_top = row(util::WEEKDAYS.map(|weekday| {
            util::colored_button(
                text(util::short_weekday(weekday).to_string())
                    .width(Length::Fill)
                    .align_x(alignment::Horizontal::Center),
                Color::from_rgb8(0x70, 0x70, 0x70),
            )
            .padding(0)
            .on_press(Message::WeekdayPressed(weekday))
            .into()
        }))
        .spacing(Self::SPACING);

        let calendar_body = column((0..Self::CALENDAR_ROWS).map(|r| {
            row((0..Self::CALENDAR_COLUMNS).map(|c| self.calendar_cell(r, c)))
                .spacing(Self::SPACING)
                .into()
        }))
        .spacing(Self::CALENDER_VERTICAL_SPACING);

        row![
            scrollable(
                column![
                    util::bold_text("Date"),
                    month_offset_year,
                    space(),
                    util::bold_text("Calendar"),
                    button(
                        text("Deselect All")
                            .width(Length::Fill)
                            .align_x(alignment::Horizontal::Center)
                    )
                    .on_press(Message::DeselectPressed),
                    calendar_top,
                    calendar_body,
                ]
                .padding(Self::PADDING)
                .spacing(Self::SPACING)
            ),
            scrollable(
                column![
                    util::bold_text("Configurations"),
                    configs_io,
                    configs_input_and_top,
                    configs_body,
                    space(),
                    util::bold_text("Result"),
                    result_body,
                ]
                .padding(Self::PADDING)
                .spacing(Self::SPACING)
                .width(Self::RIGHT_WIDTH)
            ),
        ]
        .spacing(Self::SPACING)
        .into()
    }

    fn update(&mut self, message: Message) {
        self.reset_title();

        match message {
            Message::MonthSelected(month) => {
                self.clear_cells();
                self.month_selected = Some(month)
            }
            Message::OffsetInput(offset) => {
                self.clear_cells();
                self.offset_input = offset
            }
            Message::YearInput(year) => {
                self.clear_cells();
                self.year_input = year
            }
            Message::NameInput(name) => self.name_input = name,
            Message::TypeSelected(r#type) => self.type_selected = Some(r#type),
            Message::PayInput(pay) => self.pay_input = pay,
            Message::PushPressed => match self.config() {
                Ok(config) => {
                    self.configs.insert(self.name_input.clone(), config);
                }
                Err(failure) => self.set_failure(failure),
            },
            Message::RemovePressed(name) => {
                self.configs.remove(&name);
                for cell in &mut self.cells {
                    cell.remove(&name);
                }
            }
            Message::RemoveFilePressed => {
                if let Err(failure) = self.remove_file() {
                    self.set_failure(failure);
                }

                self.filename_selected = None;
            }
            Message::HourInput(x) => self.hour_input = x,
            Message::MinuteInput(x) => self.minute_input = x,
            Message::AddPressed(name) => self
                .cells
                .iter_mut()
                .filter(|x| x.selected)
                .for_each(|x| x.insert(name.clone())),
            Message::CellChecked(b, i) => match self.get_cell_mut(i as usize) {
                Ok(cell) => cell.selected = b,
                Err(failure) => self.set_failure(failure),
            },
            Message::CellButtonPressed(name, i) => match self.get_cell_mut(i as usize) {
                Ok(cell) => cell.remove(&name),
                Err(failure) => self.set_failure(failure),
            },
            Message::DeselectPressed => self.deselect(),
            Message::FilenameInput(filename) => self.filename_input = filename,
            Message::FilenameSelected(filename) => self.filename_selected = Some(filename),
            Message::LoadPressed => match self.load() {
                Ok(configs) => {
                    self.clear_added();
                    self.configs = configs;
                }
                Err(failure) => self.set_failure(failure),
            },
            Message::SavePressed => match self.save() {
                Ok(()) => self.set_title("Configurations saved"),
                Err(failure) => self.set_failure(failure),
            },
            Message::WeekdayPressed(weekday) => {
                for i in 0..self.cells.len() {
                    let highlighted = self
                        .cell_date(i)
                        .map(|date| self.is_highlighted(&date))
                        .unwrap_or(false);
                    let column = i as u8 % Self::CALENDAR_COLUMNS;
                    let select = highlighted && (column == util::weekday_to_column(weekday));
                    if select {
                        self.get_cell_mut(i).expect("unreachable").select();
                    }
                }
            }
        }
    }
}

fn main() -> iced::Result {
    const WINDOW_SIZE: Size = Size {
        width: 1550.0,
        height: 800.0,
    };

    let palette = theme::Palette {
        background: Color::from_rgb8(31, 31, 31),
        text: Color::from_rgb8(0xFF, 0xFF, 0xFF),
        primary: Color::from_rgb8(0, 0x3F, 0x7F),
        danger: Color::from_rgb8(0xFF, 0, 0),
        success: Color::from_rgb8(0, 0xFF, 0),
    };

    iced::application(App::title, App::update, App::view)
        .theme(move |_| Theme::custom("Custom".to_string(), palette))
        .window_size(WINDOW_SIZE)
        .run()
}
