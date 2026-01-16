use time::Date;
use chinese_lunisolar_calendar::{LunarDay, LunarMonth, LunisolarDate, SolarDate};

#[derive(Debug)]
pub struct Error {
    pub message: String,
}

pub struct LunnarDate {
    pub year: String,
    pub month: String,
    pub day: String,
    pub solar_term: String,     // 24节气
}

// 24节气
const SOLAR_TERMS: [&str; 24] = [
    "小寒", "大寒", "立春", "雨水", "惊蛰", "春分", "清明", "谷雨",
    "立夏", "小满", "芒种", "夏至", "小暑", "大暑", "立秋", "处暑",
    "白露", "秋分", "寒露", "霜降", "立冬", "小雪", "大雪", "冬至",
];

// 节气计算参数（基于1900年）
const SOLAR_TERM_BASE: [f64; 24] = [
    5.4055, 20.12, 3.87, 18.74, 5.63, 20.646, 4.81, 20.1,
    5.52, 21.04, 5.678, 21.37, 7.108, 22.83, 7.5, 23.13,
    7.646, 23.042, 8.318, 23.438, 7.438, 22.36, 7.18, 21.94,
];

const SOLAR_TERM_COEFFICIENT: [f64; 24] = [
    0.2422, 0.2422, 0.2422, 0.2422, 0.2422, 0.2422, 0.2422, 0.2422,
    0.2422, 0.2422, 0.2422, 0.2422, 0.2422, 0.2422, 0.2422, 0.2422,
    0.2422, 0.2422, 0.2422, 0.2422, 0.2422, 0.2422, 0.2422, 0.2422,
];

impl LunnarDate {
    pub fn from(date: &Date) -> std::result::Result<Self, Error> {
        let mut solar_term = Self::get_solar_term(date);
        let lunisolar_date = LunisolarDate::from_solar_date(SolarDate::from_ymd(date.year() as u16, date.month() as u8, date.day()).unwrap()).unwrap();

        if lunisolar_date.to_lunar_month() == LunarMonth::First && lunisolar_date.to_lunar_day() == LunarDay::First {
            solar_term = "春节".into();
        }

        Ok(LunnarDate {
            year: format!("{}", lunisolar_date.to_lunar_year()),
            month: format!("{}", lunisolar_date.to_lunar_month()),
            day: format!("{}", lunisolar_date.to_lunar_day()),
            solar_term,
        })
    }
    // 计算24节气
    fn get_solar_term(date: &Date) -> String {
        let year = date.year();
        let month = date.month() as u8;
        let day = date.day();

        if year < 1900 || year > 2100 {
            return String::new();
        }

        // 每个月有两个节气
        let term_index = ((month - 1) * 2) as usize;

        // 计算当月的两个节气日期
        for i in 0..2 {
            let idx = term_index + i;
            if idx >= 24 {
                break;
            }

            let term_day = Self::calculate_solar_term_day(year, idx);
            if day == term_day {
                return SOLAR_TERMS[idx].to_string();
            }
        }

        String::new()
    }

    // 计算某个节气的日期
    fn calculate_solar_term_day(year: i32, term_index: usize) -> u8 {
        if term_index >= 24 {
            return 0;
        }

        let _century = year / 100;
        let year_in_century = year % 100;

        let mut day = SOLAR_TERM_BASE[term_index]
            + SOLAR_TERM_COEFFICIENT[term_index] * (year_in_century as f64)
            - ((year_in_century as f64 / 4.0).floor());

        // 特殊年份修正
        day += Self::get_term_correction(year, term_index);

        day.round() as u8
    }

    // 节气特殊年份修正
    fn get_term_correction(year: i32, term_index: usize) -> f64 {
        // 这里可以添加特殊年份的修正值
        // 简化处理，实际应该有更详细的修正表
        if year >= 2000 {
            match term_index {
                0 | 1 => -0.2, // 小寒、大寒
                2 | 3 => 0.0,  // 立春、雨水
                _ => 0.0,
            }
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Month;

    #[test]
    fn test_lunar_conversion() {
        // 测试2024年春节（2024-02-10）
        let date = Date::from_calendar_date(2026, Month::January, 10).unwrap();
        let lunar = LunnarDate::from(&date).unwrap();
        assert_eq!(lunar.day, "廿二");
    }

    #[test]
    fn test_solar_term() {
        // 测试2024年立春（2024-02-04）
        let date = Date::from_calendar_date(2024, Month::February, 4).unwrap();
        let lunar = LunnarDate::from(&date).unwrap();
        assert_eq!(lunar.solar_term, "立春");
    }
}
