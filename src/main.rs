mod component;

use component::{calendar::CalendarTracker, record::RecordTracker};
use slint::ComponentHandle;

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() -> Result<(), slint::PlatformError> {
    env_logger::init();
    let app = MainWindow::new()?;

    let calender = CalendarTracker::new();
    calender.get_date(&app);
    calender.day_of_year(&app);

    let r = RecordTracker::new();
    r.save_record_msg(&app);

    app.run()
}

#[cfg(test)]
mod test {
    #[test]
    fn f1() {}
}
