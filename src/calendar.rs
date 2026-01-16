use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Text, Span};
use ratatui::widgets::calendar::{CalendarEventStore};
use ratatui::{DefaultTerminal, Frame};
use time::ext::NumericalDuration;
use time::{Date, Month, OffsetDateTime};

// 日历
pub struct Calendar {

}

impl Calendar {
    pub fn new() -> Self {
        Calendar {

        }
    }

    pub fn run(self, mut terminal: DefaultTerminal) -> Result<()> {
        let mut selected_date = OffsetDateTime::now_local()?.date();
        loop {
            terminal.draw(|frame| self.render(frame, selected_date))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') => break Ok(()),
                    KeyCode::Char('n') | KeyCode::Tab => selected_date = self.next_month(selected_date),
                    KeyCode::Char('p') | KeyCode::BackTab => selected_date = self.prev_month(selected_date),
                    KeyCode::PageUp => selected_date = self.prev_year(selected_date),
                    KeyCode::PageDown => selected_date = self.next_year(selected_date),
                    KeyCode::Char('h') | KeyCode::Left => selected_date -= 1.days(),
                    KeyCode::Char('j') | KeyCode::Down => selected_date += 1.weeks(),
                    KeyCode::Char('k') | KeyCode::Up => selected_date -= 1.weeks(),
                    KeyCode::Char('l') | KeyCode::Right => selected_date += 1.days(),
                    KeyCode::Char('t') => selected_date = OffsetDateTime::now_local()?.date(),
                    _ => {}
                }
            }
        }
    }

    fn next_month(&self, date: Date) -> Date {
        if date.month() == Month::December {
            date.replace_month(Month::January)
                .unwrap()
                .replace_year(date.year() + 1)
                .unwrap()
        } else {
            date.replace_month(date.month().next()).unwrap()
        }
    }

    fn prev_month(&self, date: Date) -> Date {
        if date.month() == Month::January {
            date.replace_month(Month::December)
                .unwrap()
                .replace_year(date.year() - 1)
                .unwrap()
        } else {
            date.replace_month(date.month().previous()).unwrap()
        }
    }
    fn next_year(&self, date: Date) -> Date {
        // 最大只能显示到 2100 年
        if date.year() >= 2100 {
            return date;
        }
        date.replace_year(date.year() + 1).unwrap()
    }
    fn prev_year(&self, date: Date) -> Date {
        // 最小只能显示到 1900 年
        if date.year() <= 1900 {
            return date;
        }
        date.replace_year(date.year() - 1).unwrap()
    }

    fn render(&self, frame: &mut Frame, selected_date: Date) {
        let header = Text::from_iter([
            Line::from("中华万年历".bold()),
            Line::from("<q> 退出 | <n> 下个月, <p> 上个月 | <PageDown> 下一年, <PageUp> 上一年"),
            Line::from("<hjkl←↓↑→> 移动选择日期 | <t> 跳转到今日"),
            Line::from_iter([
                Span::raw("当前日期："),
                Span::styled(format!("{}", selected_date), Style::default().fg(Color::Yellow)),
                Span::raw(" | 日期颜色："),
                Span::styled("放假", Style::default().bg(crate::render::month::HOLIDAY_COLOR)),
                Span::styled("加班", Style::default().bg(crate::render::month::WORKDAY_COLOR)),
                Span::styled("今天", Style::default().bg(crate::render::month::TODAY_COLOR)),
            ]),
        ]);

        let [text_area, area] = frame.area().layout(&Layout::vertical([
                Constraint::Length(header.height() as u16 + 1),
                Constraint::Max(45),
        ]));

        frame.render_widget(header.centered(), text_area);


        let mut event_list = CalendarEventStore::today(
            Style::default()
            .add_modifier(Modifier::UNDERLINED)
        );
        event_list.add(selected_date, Style::default().bg(crate::render::month::TODAY_COLOR));
        // 显示月份
        let month_line_blocks: [Rect; 3] = area.layout(&Layout::vertical([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ]));

        for (index, line_block) in month_line_blocks.iter().enumerate() {
            // 一行显示4个月
            let blocks = &Layout::horizontal([
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ]).flex(ratatui::layout::Flex::SpaceBetween).split(*line_block);

            for month_offset in 1..5 {
                let month = Month::try_from(index as u8 * 4 + month_offset).unwrap();
                let month_area = blocks.get(month_offset as usize - 1).unwrap();
                crate::render::month::MonthDrawer{
                    year: selected_date.year(),
                    month,
                }.render_month(frame, *month_area, &event_list);
            }
        }
    }
}
