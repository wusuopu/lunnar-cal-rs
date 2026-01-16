use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Text, Span};
use ratatui::widgets::calendar::{CalendarEventStore, DateStyler, Monthly};
use ratatui::{Frame};
use time::{Date, Month, OffsetDateTime};

pub const HOLIDAY_COLOR: Color = Color::Red;
pub const WORKDAY_COLOR: Color = Color::LightMagenta;
pub const TODAY_COLOR: Color = Color::LightBlue;
pub const SEASON_COLOR: Color = Color::Green;

pub struct MonthDrawer {
    pub year: i32,
    pub month: Month,
}

impl MonthDrawer {
    pub fn render_month(self, frame: &mut Frame, area: Rect, events: &CalendarEventStore) {
        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1)
        ]);
        let [header_area, weekheaer, main_area] = area.layout(&layout);

        let header = Line::from(format!("{} {}月", self.year, self.month as u8)).bold();
        frame.render_widget(header.centered(), header_area);

        self.render_weekheader(frame, weekheaer);

        let mut day_text: Vec<Line> = Vec::new();
        let mut line1: Vec<Span> = Vec::new();
        let mut line2: Vec<Span> = Vec::new();
        for day in 1..32 {
            let date = Date::from_calendar_date(self.year, self.month, day);
            if date.is_err() {
                break;
            }

            let date = date.unwrap();
            let offset = date.weekday().number_days_from_sunday();
            if offset > 0 && date.day() == 1 {      // 每月1日之前补空格
                for _ in 0..offset {
                    line1.push(Span::styled(format!("{:^6}", " "), Style::default().bg(Color::DarkGray)));
                    line2.push(Span::styled(format!("{:^6}", " "), Style::default().bg(Color::DarkGray)));
                }
            }

            line1.push(self.build_day_span(&date, events, false));
            line2.push(self.build_day_span(&date, events, true));

            if line1.len() == 7 {
                day_text.push(Line::from_iter(line1.clone()));
                day_text.push(Line::from_iter(line2.clone()));
                line1.clear();
                line2.clear();
            }
        }
        if !line1.is_empty() {
            for _ in 0..7 - line1.len() {
                line1.push(Span::styled(format!("{:^6}", " "), Style::default().bg(Color::DarkGray)));
                line2.push(Span::styled(format!("{:^6}", " "), Style::default().bg(Color::DarkGray)));
            }
            day_text.push(Line::from_iter(line1.clone()));
            day_text.push(Line::from_iter(line2.clone()));
            line1.clear();
            line2.clear();
        }

        frame.render_widget(Text::from_iter(day_text).centered(), main_area);
    }

    fn render_weekheader(&self, frame: &mut Frame, area: Rect) {
        let header = Line::from_iter([
            Span::styled(format!("{:^5}", "日"), Style::default().bold().fg(HOLIDAY_COLOR)),
            Span::styled(format!("{:^5}", "一"), Style::default().bold()),
            Span::styled(format!("{:^5}", "二"), Style::default().bold()),
            Span::styled(format!("{:^5}", "三"), Style::default().bold()),
            Span::styled(format!("{:^5}", "四"), Style::default().bold()),
            Span::styled(format!("{:^5}", "五"), Style::default().bold()),
            Span::styled(format!("{:^5}", "六"), Style::default().bold().fg(HOLIDAY_COLOR)),
        ]);
        frame.render_widget(header.centered(), area);
    }

    fn build_day_span(&self, day: &Date, events: &CalendarEventStore, is_lunar: bool) -> Span {
        let mut style = events.get_style(day.clone());
        if style.fg.is_none() {
            if is_lunar {
                style = style.fg(Color::Black);
            } else {
                style = style.fg(Color::White).bold();
            }
        }
        if style.bg.is_none() {
            style = style.bg(Color::DarkGray);
        }
        if is_lunar {
            let date = crate::utils::lunnar::LunnarDate::from(&day.clone()).unwrap();
            let value = if date.solar_term.is_empty() {
                date.day
            } else {
                date.solar_term
            };
            return Span::styled(format!("{:^4}", value), style);
        }
        return Span::styled(format!("{:^6}", day.day()), style);
    }
}
