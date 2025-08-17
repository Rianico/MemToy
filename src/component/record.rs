use log::{debug, error};
use slint::ComponentHandle;

use crate::component::calendar::DATE_FORMAT;
use crate::component::data_management::{DataManagementType, General};
use crate::{MainWindow, RecordController, RecordRes};

pub(crate) struct RecordTracker {
    management: DataManagementType,
}

impl RecordTracker {
    pub(crate) fn new() -> Self {
        Self {
            management: DataManagementType::Simple(General::new()),
        }
    }

    pub(crate) fn save_record_msg(&self, app: &MainWindow) {
        let managenet = self.management;
        app.global::<RecordController>().on_save_records(move |record, date| {
            let record = record.as_str();
            debug!("saved record: {record}, create_date: {date:?}, checkpoint_date: {date:?}");
            if record.is_empty() {
                return RecordRes {
                    success: false,
                    msg: "text can't be empty".into(),
                };
            }
            match managenet {
                DataManagementType::Simple(ref general) => general
                    .save_records(record, chrono::NaiveDate::parse_from_str(&date, DATE_FORMAT).ok())
                    .map_or_else(
                        |msg| {
                            error!("fail to save record, error: {msg:?}");
                            RecordRes {
                                success: false,
                                msg: "save failed".into(),
                            }
                        },
                        |msg| RecordRes {
                            success: true,
                            msg: msg.into(),
                        },
                    ),
            }
        });
    }
}
