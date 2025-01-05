mod component;

use component::calendar::CalendarTracker;
use component::record::RecordTracker;
use component::review::ReviewTracker;

use slint::ComponentHandle;

slint::include_modules!();

pub fn main() -> Result<(), slint::PlatformError> {
    env_logger::init();

    let app = MainWindow::new()?;

    let calendar = CalendarTracker::new();
    calendar.get_date(&app);
    calendar.day_of_year(&app);

    let recordd = RecordTracker::new();
    recordd.save_record_msg(&app);

    let review = ReviewTracker::new();
    review.review_today(&app);

    app.run()
}
