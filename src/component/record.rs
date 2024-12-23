use log::debug;
use log::error;
use rusqlite::Connection;
use slint::ComponentHandle;

use crate::component::calendar::DATE_FORMAT;
use crate::component::data_management::DataManagement;
use crate::component::data_management::DataManagementType;
use crate::component::data_management::General;
use crate::component::data_management::DB_FILE;
use crate::MainWindow;
use crate::RecordController;
use crate::RecordRes;

pub(crate) struct RecordTracker {
    management: DataManagementType,
}

const RECORD_TABALE_SQL: &str = "CREATE TABLE IF NOT EXISTS records (
  id INTEGER PRIMARY KEY,
  content TEXT,
  create_date DATE
)";

const IDX_RECORD_DATE: &str =
    "CREATE INDEX IF NOT EXISTS idx_record_create_date ON records (create_date)";

impl RecordTracker {
    pub(crate) fn new() -> Self {
        let con = DB_FILE
            .with(|p| Connection::open(p))
            .unwrap_or_else(|e| panic!("Failed to open the database file: {e:?}"));
        debug!("Create table: {RECORD_TABALE_SQL}");
        con.execute(RECORD_TABALE_SQL, []).unwrap_or_else(|msg| {
            panic!("Fail to create the initialized table, error: {msg}, sql: {RECORD_TABALE_SQL}")
        });
        debug!("Create inddex: {IDX_RECORD_DATE}");
        con.execute(IDX_RECORD_DATE, []).unwrap_or_else(|msg| {
            panic!("Fail to create index, error: {msg}, sql: {IDX_RECORD_DATE}")
        });
        Self {
            management: DataManagementType::Simple(General::new()),
        }
    }

    pub(crate) fn save_record_msg(&self, app: &MainWindow) {
        let managenet = self.management;
        app.global::<RecordController>()
            .on_save(move |record, date| {
                let record = record.as_str();
                debug!("saved record: {record}, create_date: {date:?}");
                if record.is_empty() {
                    return RecordRes {
                        success: false,
                        msg: "text can't be empty".into(),
                    };
                }
                match managenet {
                    DataManagementType::Simple(ref general) => match general.save(
                        record,
                        chrono::NaiveDate::parse_from_str(&date, DATE_FORMAT).ok(),
                    ) {
                        Ok(msg) => RecordRes {
                            success: true,
                            msg: msg.into(),
                        },
                        Err(msg) => {
                            error!("fail to save record, error: {msg:?}");
                            RecordRes {
                                success: false,
                                msg: "save failed".into(),
                            }
                        }
                    },
                }
            });
    }
}
