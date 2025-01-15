use std::rc::Rc;

use log::{debug, error, info};
use regex::Regex;
use rusqlite::Connection;
use slint::{ComponentHandle, ModelRc, VecModel};
use url::Url;

use super::data_management::{DataManagementType, General};
use crate::component::data_management::DB_FILE;
use crate::{MainWindow, RecordRes, ReviewController};

pub(crate) struct ReviewTracker {
    management: DataManagementType,
}

const TASK_TABLE_SQL: &str = "CREATE TABLE IF NOT EXISTS tasks (
  id INTEGER NOT NULL,
  create_date DATE NOT NULL,
  finished BOOLEAN NOT NULL,
  PRIMARY KEY (id, create_date)
)";

impl ReviewTracker {
    pub(crate) fn new() -> Self {
        let con = DB_FILE
            .with(|p| Connection::open(p))
            .unwrap_or_else(|e| panic!("Failed to open the database file: {e:?}"));
        debug!("Create task table: {TASK_TABLE_SQL}");
        con.execute(TASK_TABLE_SQL, [])
            .unwrap_or_else(|msg| panic!("Fail to create the initialized table, error: {msg}, sql: {TASK_TABLE_SQL}"));
        Self {
            management: DataManagementType::Simple(General::new()),
        }
    }
}

impl ReviewTracker {
    pub(crate) fn review_today(&self, app: &MainWindow) {
        let managenet = self.management;
        app.global::<ReviewController>().on_review_today(move || match managenet {
            DataManagementType::Simple(ref general) => match general.query_today_review() {
                Ok(records) => ModelRc::from(Rc::new(VecModel::from(records))),
                Err(msg) => {
                    error!("Query records failed, error: {msg}");
                    ModelRc::default()
                }
            },
        });
    }

    pub(crate) fn toggle_task(&self, app: &MainWindow) {
        let managenet = self.management;
        app.global::<ReviewController>().on_toggle_task(move |id, finished| match managenet {
            DataManagementType::Simple(ref general) => match general.toggle_task(id, finished) {
                Ok(()) => {
                    info!("change task finished status to {}", finished);
                }
                Err(msg) => {
                    error!("change task finished status failed, err: {}", msg)
                }
            },
        })
    }

    pub(crate) fn del_task(&self, app: &MainWindow) {
        let managenet = self.management;
        app.global::<ReviewController>().on_del_task(move |id| match managenet {
            DataManagementType::Simple(ref general) => match general.del_record_and_task(id) {
                Ok(()) => {
                    info!("delete record and task, id: {id}");
                }
                Err(msg) => {
                    error!("delete record and task failed, err: {}", msg)
                }
            },
        })
    }

    pub(crate) fn update_task(&self, app: &MainWindow) {
        let managenet = self.management;
        app.global::<ReviewController>().on_update_task(move |id, content| {
            let content = content.as_str();
            debug!("update record: {content}, id: {id}");
            if content.is_empty() {
                return RecordRes {
                    success: false,
                    msg: "text can't be empty".into(),
                };
            }
            match managenet {
                DataManagementType::Simple(ref general) => match general.update_record(id, content) {
                    Ok(msg) => RecordRes {
                        success: true,
                        msg: msg.into(),
                    },
                    Err(msg) => {
                        error!("fail to update record, error: {msg:?}");
                        RecordRes {
                            success: false,
                            msg: "save failed".into(),
                        }
                    }
                },
            }
        })
    }

    pub(crate) fn open_link(&self, app: &MainWindow) {
        app.global::<ReviewController>().on_open_link(|content| {
            let re = Regex::new(r"https?://[^\s]+").unwrap();
            if let Some(url) = re
                .captures(content.as_str())
                .and_then(|captured| captured.get(0))
                .and_then(|url_match| Url::parse(url_match.as_str()).ok())
            {
                if webbrowser::open(url.as_str()).is_ok() {
                    info!("Successfully opened the URL in the browser.");
                } else {
                    error!("Failed to open the URL.");
                }
            }
        })
    }
}
