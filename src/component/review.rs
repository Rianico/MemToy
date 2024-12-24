use std::rc::Rc;

use log::{debug, error};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

use super::data_management::{DataManagementType, General};

use crate::{MainWindow, ReviewController};

pub(crate) struct ReviewTracker {
    management: DataManagementType,
}

impl ReviewTracker {
    pub(crate) fn new() -> Self {
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
                    match general.query_today_review::<SharedString, String>() {
                        Ok(records) => {
                            debug!("records: {records:?}");
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
}
