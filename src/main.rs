mod component;

use component::{calendar::CalendarTracker, record::RecordTracker, review::ReviewTracker};
use slint::ComponentHandle;

slint::include_modules!();

pub fn main() -> Result<(), slint::PlatformError> {
    env_logger::init();
    let app = MainWindow::new()?;

    let calender = CalendarTracker::new();
    calender.get_date(&app);
    calender.day_of_year(&app);

    let recordd = RecordTracker::new();
    recordd.save_record_msg(&app);

    let review = ReviewTracker::new();
    review.review_today(&app);

    app.run()
}
