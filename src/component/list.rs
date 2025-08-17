use std::rc::Rc;

use log::{debug, error, info};
use regex::Regex;
use slint::{ComponentHandle, ModelRc, VecModel};
use url::Url;

use super::data_management::{DataManagementType, General};
use crate::{ListController, MainWindow, RecordRes};

pub(crate) struct ListTracker {
    management: DataManagementType,
}

impl ListTracker {
    pub(crate) fn new() -> Self {
        Self {
            management: DataManagementType::Simple(General::new()),
        }
    }
}

impl ListTracker {
    pub(crate) fn query_all_records(&self, app: &MainWindow) {
        debug!("query_all_records");
        let managenet = self.management;
        app.global::<ListController>().on_all_records(move || {
            debug!("match general");
            match managenet {
                DataManagementType::Simple(ref general) => match general.query_all_records() {
                    Ok(records) => ModelRc::from(Rc::new(VecModel::from(records))),
                    Err(msg) => {
                        error!("Query records failed, error: {msg}");
                        ModelRc::default()
                    }
                },
            }
        });
        debug!("query_all_records end");
    }

    pub(crate) fn del_record(&self, app: &MainWindow) {
        let managenet = self.management;
        app.global::<ListController>().on_del_record(move |id| match managenet {
            DataManagementType::Simple(ref general) => match general.del_record_and_task(id) {
                Ok(()) => {
                    info!("delete record and record, id: {id}");
                }
                Err(msg) => {
                    error!("delete record and record failed, err: {msg}")
                }
            },
        })
    }

    pub(crate) fn update_record(&self, app: &MainWindow) {
        let managenet = self.management;
        app.global::<ListController>().on_update_record(move |id, content| {
            let content = content.as_str();
            debug!("update record: {content}, id: {id}");
            if content.is_empty() {
                return RecordRes {
                    success: false,
                    msg: "text can't be empty".into(),
                };
            }
            match managenet {
                DataManagementType::Simple(ref general) => general
                    .update_record(id, content)
                    .map(|msg| RecordRes {
                        success: true,
                        msg: msg.into(),
                    })
                    .unwrap_or_else(|msg| {
                        error!("fail to update record, error: {msg:?}");
                        RecordRes {
                            success: false,
                            msg: "save failed".into(),
                        }
                    }),
            }
        })
    }

    pub(crate) fn refresh_checkpoint(&self, app: &MainWindow) {
        let managenet = self.management;
        app.global::<ListController>().on_refresh_checkpoint(move |id| {
            debug!("refresh record, id: {id}");
            match managenet {
                DataManagementType::Simple(ref general) => general
                    .refresh_checkpoint(id)
                    .map(|msg| RecordRes {
                        success: true,
                        msg: msg.into(),
                    })
                    .unwrap_or_else(|msg| {
                        error!("fail to refreh record checkpoint day, error: {msg:?}");
                        RecordRes {
                            success: false,
                            msg: "save failed".into(),
                        }
                    }),
            }
        })
    }

    pub(crate) fn open_link(&self, app: &MainWindow) {
        app.global::<ListController>().on_open_link(|content| {
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
