#![allow(unused)]
use std::env;
use std::path::Path;
use std::path::PathBuf;

use log::debug;
use log::info;
use rusqlite::{params, Connection};
use slint::ComponentHandle;

use crate::MainWindow;
use crate::RecordController;
use crate::RecordRes;

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
        app.global::<RecordController>().on_save(move |record| {
            let record = record.as_str();
            debug!("{record}");
            if record.is_empty() {
                return RecordRes {
                    success: false,
                    msg: "text can't be empty".into(),
                };
            }
            match managenet {
                DataManagementType::Simple(ref general) => match general.save(record) {
                    Ok(msg) => RecordRes {
                        success: true,
                        msg: "save success".into(),
                    },
                    Err(_) => RecordRes {
                        success: false,
                        msg: "save failed".into(),
                    },
                },
                _ => unreachable!(),
            }
        });
    }
}

const RECORD_TABALE_SQL: &str = "CREATE TABLE IF NOT EXISTS records (
  id INTEGER PRIMARY KEY,
  content TEXT
)";

#[derive(Clone, Copy)]
enum DataManagementType {
    Simple(General),
}
trait DataManagement {
    fn save(&self, data: impl AsRef<str>) -> anyhow::Result<()> {
        let db_file = env::current_dir()?.join("data").join("record.db");
        let con = Connection::open(db_file)?;
        info!("Create table: {RECORD_TABALE_SQL}");
        con.execute(RECORD_TABALE_SQL, [])?;
        con.execute("INSERT INTO records (content) VALUES (?1)", [data.as_ref()])?;
        Ok(())
    }

    fn query<'a>(condition: impl Into<&'a str>) -> anyhow::Result<Vec<String>>;
}

#[derive(Clone, Copy)]
struct General;

impl General {
    fn new() -> Self {
        General {}
    }
}

impl DataManagement for General {
    fn query<'a>(condition: impl Into<&'a str>) -> anyhow::Result<Vec<String>> {
        todo!()
    }
}
