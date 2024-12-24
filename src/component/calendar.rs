use crate::CalendarController;
use crate::MainWindow;

use slint::ComponentHandle;

use chrono::NaiveDate;

pub(crate) const DATE_FORMAT: &str = "%a, %Y-%m-%d";
const DAY_OF_YEAR_FORMAT: &str = "Today is the %jth Day";

pub(crate) struct CalendarTracker;

impl CalendarTracker {
    pub fn new() -> Self {
        CalendarTracker
    }

    pub fn get_date(&self, app: &MainWindow) {
        app.global::<CalendarController>()
            .on_get_date(move |year, month, day| {
                if year == 0 && month == 0 && day == 0 {
                    chrono::Local::now()
                        .date_naive()
                        .format(DATE_FORMAT)
                        .to_string()
                        .into()
                } else {
                    NaiveDate::from_ymd_opt(year, month as u32, day as u32)
                        .map(|date| date.format(DATE_FORMAT).to_string())
                        .unwrap_or("error year, month, day".to_string())
                        .into()
                }
            });
    }

    pub fn day_of_year(&self, app: &MainWindow) {
        app.global::<CalendarController>()
            .on_day_of_year(|year, month, day| {
                if year == 0 && month == 0 && day == 0 {
                    chrono::Local::now()
                        .date_naive()
                        .format(DAY_OF_YEAR_FORMAT)
                        .to_string()
                        .into()
                } else {
                    NaiveDate::from_ymd_opt(year, month as u32, day as u32)
                        .map(|date| date.format(DAY_OF_YEAR_FORMAT).to_string())
                        .unwrap_or("error year, month, day".to_string())
                        .into()
                }
            })
    }
}
