use iced::{Background, Border, Color, Element, Font, Theme, border, font, theme, widget};
use std::{
    collections::VecDeque,
    hash::{DefaultHasher, Hasher},
};
use time::{Date, Month, Weekday};

pub trait Some {
    fn some(self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(self)
    }
}

impl<T> Some for T {}

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

pub const fn short_month(month: Month) -> &'static str {
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

pub const fn short_weekday(weekday: Weekday) -> &'static str {
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

pub fn colored_button<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    color: Color,
) -> widget::Button<'a, Message> {
    use widget::{
        button,
        button::{Status, Style},
    };

    // copied from source
    fn styled(pair: theme::palette::Pair) -> Style {
        Style {
            background: Background::Color(pair.color).some(),
            text_color: pair.text,
            border: rounded_border(),
            ..Style::default()
        }
    }

    // copied from source
    fn disabled(style: Style) -> Style {
        Style {
            background: style
                .background
                .map(|background| background.scale_alpha(0.5)),
            text_color: style.text_color.scale_alpha(0.5),
            ..style
        }
    }

    button(content).style(move |_, status| {
        let theme = Theme::custom(
            "Button".to_string(),
            theme::Palette {
                primary: color,
                text: Color::from_rgb8(0xFF, 0xFF, 0xFF),

                // these three don't matter
                danger: Default::default(),
                success: Default::default(),
                background: Default::default(),
            },
        );

        let palette = theme.extended_palette();
        let base = styled(palette.primary.base);

        match status {
            Status::Active | Status::Pressed => base,
            Status::Hovered => Style {
                background: Background::Color(palette.primary.strong.color).some(),
                ..base
            },
            Status::Disabled => disabled(base),
        }
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

pub const fn weekday_to_column(weekday: Weekday) -> u8 {
    match weekday {
        Weekday::Sunday => 0,
        Weekday::Monday => 1,
        Weekday::Tuesday => 2,
        Weekday::Wednesday => 3,
        Weekday::Thursday => 4,
        Weekday::Friday => 5,
        Weekday::Saturday => 6,
    }
}

pub fn current_date() -> Option<Date> {
    use std::time::{SystemTime, UNIX_EPOCH};
    use time::UtcDateTime;

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|timestamp| UtcDateTime::from_unix_timestamp(timestamp.as_secs() as i64).ok())
        .map(|x| x.date())
}
