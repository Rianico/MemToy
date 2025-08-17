mod component;

use component::calendar::CalendarTracker;
use component::db_checker::DbChecker;
use component::list::ListTracker;
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

    DbChecker::create_check_db();

    let recordd = RecordTracker::new();
    recordd.save_record_msg(&app);

    let review = ReviewTracker::new();
    review.review_today(&app);
    review.toggle_task(&app);
    review.del_task(&app);
    review.update_task(&app);
    review.refresh_checkpoint(&app);
    review.open_link(&app);

    let list = ListTracker::new();
    list.query_all_records(&app);
    list.del_record(&app);
    list.update_record(&app);
    list.refresh_checkpoint(&app);
    list.open_link(&app);

    app.run()
}
