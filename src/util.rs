use iced::{Background, Border, Color, Element, Font, border, font, widget};
use std::{
    collections::VecDeque,
    hash::{DefaultHasher, Hasher},
};
use time::{Month, Weekday};

pub const MONTHS: [Month; 12] = [
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

pub const WEEKDAYS: [Weekday; 7] = [
    Weekday::Sunday,
    Weekday::Monday,
    Weekday::Tuesday,
    Weekday::Wednesday,
    Weekday::Thursday,
    Weekday::Friday,
    Weekday::Saturday,
];

pub const fn short_month(month: &Month) -> &str {
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

pub const fn short_weekday(weekday: &Weekday) -> &str {
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

pub fn rounded_container<'a, Message>(
    content: impl Into<Element<'a, Message>>,
) -> widget::Container<'a, Message> {
    use widget::container;

    container(content).style(container::rounded_box)
}

pub fn bold_text<'a>(text: impl widget::text::IntoFragment<'a>) -> widget::Text<'a> {
    widget::text(text).font(Font {
        weight: font::Weight::Bold,
        ..Default::default()
    })
}

pub fn monospace_text<'a>(text: impl widget::text::IntoFragment<'a>) -> widget::Text<'a> {
    widget::text(text).font(Font {
        family: font::Family::Monospace,
        ..Default::default()
    })
}

pub fn colored_thin_button<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    color: Color,
) -> widget::Button<'a, Message> {
    use widget::button;

    button(content).style(move |_, _| button::Style {
        background: Some(Background::Color(color)),
        text_color: Color::WHITE,
        border: rounded_border(),
        ..Default::default()
    })
}

pub fn comma_separated(n: u32) -> String {
    let mut buf = VecDeque::new();

    for (i, char) in n.to_string().chars().rev().enumerate() {
        if i != 0 && i % 3 == 0 {
            buf.push_front(',');
        }
        buf.push_front(char);
    }

    buf.iter().collect()
}

pub fn yen(n: u32) -> String {
    format!("{} ¥", comma_separated(n))
}

pub fn get_color(text: &str) -> Color {
    let mut hasher = DefaultHasher::new();

    for byte in text.bytes() {
        hasher.write_u8(byte);
    }

    let value = hasher.finish();

    let f = |x| x as u8 / 3 + 38;

    Color::from_rgb8(f(value >> 16), f(value >> 8), f(value))
}

pub fn rounded_border() -> Border {
    border::rounded(2)
}
