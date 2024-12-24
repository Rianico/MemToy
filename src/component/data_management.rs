use std::{env, fmt::Debug, path::PathBuf};

use chrono::NaiveDate;
use log::debug;
use rusqlite::{params_from_iter, types::FromSql, Connection, Params, Row};

thread_local! {
    pub static DB_FILE: PathBuf = env::current_dir()
                .map(|path| path.join("data").join("record.db"))
                .expect("Get Current Dir failed.");
}

#[derive(Clone, Copy)]
pub enum DataManagementType {
    Simple(General),
}

pub struct DataManagement;

impl DataManagement {
    fn save(data: impl AsRef<str>, create_date: Option<NaiveDate>) -> anyhow::Result<&'static str> {
        let con = DB_FILE.with(|p| Connection::open(p))?;
        con.execute(
            "INSERT INTO records (content, create_date) VALUES (?1, ?2)",
            [
                data.as_ref(),
                create_date
                    .unwrap_or(chrono::Local::now().date_naive())
                    .to_string()
                    .as_str(),
            ],
        )?;
        Ok("save success")
    }

    fn query<T, U, P, F>(
        query_sql: impl AsRef<str>,
        params: P,
        mapped_fn: F,
    ) -> anyhow::Result<Vec<T>>
    where
        T: From<U>,
        U: FromSql,
        P: Params + Debug,
        F: FnMut(&Row<'_>) -> rusqlite::Result<U>,
    {
        let con = DB_FILE.with(|p| Connection::open(p))?;
        debug!("query: {}, params: {:?}", query_sql.as_ref(), params);
        let mut stmt = con.prepare(query_sql.as_ref())?;
        let rows = stmt.query_map(params, mapped_fn)?;
        Ok(rows.into_iter().map(|r| r.unwrap().into()).collect())
    }
}

#[derive(Clone, Copy)]
pub struct General;

impl General {
    pub fn new() -> Self {
        General {}
    }

    pub fn save(
        &self,
        data: impl AsRef<str>,
        create_date: Option<NaiveDate>,
    ) -> Result<&'static str, anyhow::Error> {
        DataManagement::save(data, create_date)
    }

    // ST: Slint Type
    // RT: Rust Type
    pub fn query_today_review<ST: From<RT>, RT: FromSql>(&self) -> Result<Vec<ST>, anyhow::Error> {
        let today = chrono::Local::now().date_naive();
        let filter = [
            today.to_string(),
            (today - chrono::Duration::days(1)).to_string(),
            (today - chrono::Duration::days(3)).to_string(),
            (today - chrono::Duration::days(7)).to_string(),
            (today - chrono::Duration::days(14)).to_string(),
            (today - chrono::Duration::days(30)).to_string(),
            (today - chrono::Duration::days(60)).to_string(),
        ];
        let condition = "?,".repeat(filter.len());
        let condition = condition
            .strip_suffix(",")
            .unwrap_or_else(|| panic!("Strip suffix ',' for condition error"));
        let query_sql = format!(
            "select content from records where create_date in ({})",
            condition
        );
        DataManagement::query::<ST, RT, _, _>(query_sql, params_from_iter(&filter), |row| {
            row.get::<_, RT>(0)
        })
    }
}
