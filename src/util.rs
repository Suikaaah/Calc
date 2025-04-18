use std::collections::VecDeque;

use iced::{Element, Font, font, widget};
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

pub fn rounded_container<'a, Message: 'a>(
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
