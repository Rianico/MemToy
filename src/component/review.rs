use std::rc::Rc;

use log::{debug, error, info};
use rusqlite::Connection;
use slint::{ComponentHandle, ModelRc, VecModel};

use super::data_management::{DataManagementType, General};
use crate::{component::data_management::DB_FILE, MainWindow, ReviewController};

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
        con.execute(TASK_TABLE_SQL, []).unwrap_or_else(|msg| {
            panic!("Fail to create the initialized table, error: {msg}, sql: {TASK_TABLE_SQL}")
        });
        Self {
            management: DataManagementType::Simple(General::new()),
        }
    }
}

impl ReviewTracker {
    pub(crate) fn review_today(&self, app: &MainWindow) {
        let managenet = self.management;
        app.global::<ReviewController>()
            .on_review_today(move || match managenet {
                DataManagementType::Simple(ref general) => {
                    match general.query_today_review() {
                        Ok(records) => {
                            ModelRc::from(Rc::new(VecModel::from(records)))
                        }
                        Err(msg) => {
                            error!("Query records failed, error: {msg}");
                            ModelRc::default()
                        }
                    }
                }
            });
    }

    pub(crate) fn toggle_task(&self, app: &MainWindow) {
        let managenet = self.management;
        app.global::<ReviewController>()
            .on_toggle_task(move |id, finished|{
                match managenet {
                    DataManagementType::Simple(ref general) => {
                        match general.toggle_task(id, finished) {
                            Ok(()) => {
                                info!("change task finished status to {}", finished);
                            }
                            Err(msg) => {
                                error!("change task finished status failed, err: {}", msg)
                            }
                        }

                    },
                }
            })
    }

    pub(crate) fn del_task(&self, app: &MainWindow) {
        let managenet = self.management;
        app.global::<ReviewController>()
            .on_del_task(move |id|{
                match managenet {
                    DataManagementType::Simple(ref general) => {
                        match general.del_task(id) {
                            Ok(()) => {
                                info!("delete record and task, id: {id}");
                            }
                            Err(msg) => {
                                error!("delete record and task failed, err: {}", msg)
                            }
                        }

                    },
                }
            })
    }
}
