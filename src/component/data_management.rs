use std::fmt::Debug;
use std::path::PathBuf;

use chrono::NaiveDate;
use directories::ProjectDirs;
use log::{debug, info, trace};
use rusqlite::{params_from_iter, Connection, Params, Row};

use crate::Task;

thread_local! {
    pub static DB_FILE: PathBuf = {
        let db_path = ProjectDirs::from_path("MemToy".into())
                .expect("error occured when identify the data directory")
                .data_dir()
                .to_path_buf();
        info!("db file path is {db_path:?}");
        db_path
    }
}

#[derive(Clone, Copy)]
pub enum DataManagementType {
    Simple(General),
}

pub struct DataManagement;

impl DataManagement {
    fn save_record(
        data: impl AsRef<str>,
        create_date: Option<NaiveDate>,
    ) -> anyhow::Result<&'static str> {
        let con = DB_FILE.with(|db_file| Connection::open(db_file))?;
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

    fn update_record(id: i32, record: impl AsRef<str>) -> anyhow::Result<&'static str> {
        let con = DB_FILE.with(|db_file| Connection::open(db_file))?;
        con.execute(
            "UPDATE records set content = ?1 where id = ?2",
            (record.as_ref(), &id),
        )?;
        Ok("save success")
    }

    fn del_task(id: i32) -> anyhow::Result<()> {
        let mut con = DB_FILE.with(|db_file| Connection::open(db_file))?;
        let tx = con.transaction()?;
        // Use `INSERT OR REPLACE` to update or insert the record
        tx.execute("delete from records where id = ?", [id])?;
        tx.execute("delete from tasks where id = ?", [id])?;
        tx.commit()?;
        Ok(())
    }

    fn toggle_task(id: i32, finished: bool) -> anyhow::Result<()> {
        let con = DB_FILE.with(|db_file| Connection::open(db_file))?;
        // Use `INSERT OR REPLACE` to update or insert the record
        con.execute(
            "INSERT OR REPLACE INTO tasks (id, create_date, finished) VALUES (?1, ?2, ?3)",
            (
                &id,
                chrono::Local::now().date_naive().to_string().as_str(),
                &finished,
            ),
        )?;
        Ok(())
    }

    fn query<P, F, T>(query_sql: impl AsRef<str>, params: P, mapped_fn: F) -> anyhow::Result<Vec<T>>
    where
        P: Params + Debug,
        F: FnMut(&Row<'_>) -> rusqlite::Result<T> + 'static,
        T: Debug,
    {
        let con = DB_FILE.with(|p| Connection::open(p))?;
        debug!("query: {}, params: {:?}", query_sql.as_ref(), params);
        let mut stmt = con.prepare(query_sql.as_ref())?;
        debug!(
            "column nums: {}, column names: {:?}",
            stmt.column_count(),
            stmt.column_names()
        );
        let mut rows = stmt.query_map(params, mapped_fn)?.peekable();
        trace!("{:?}", rows.peek());
        Ok(rows.map(|r| r.unwrap()).collect())
    }
}

#[derive(Clone, Copy)]
pub struct General;

impl General {
    pub fn new() -> Self {
        General {}
    }

    pub fn save_records(
        &self,
        data: impl AsRef<str>,
        create_date: Option<NaiveDate>,
    ) -> Result<&'static str, anyhow::Error> {
        DataManagement::save_record(data, create_date)
    }

    pub fn update_record(
        &self,
        id: i32,
        record: impl AsRef<str>,
    ) -> Result<&'static str, anyhow::Error> {
        DataManagement::update_record(id, record)
    }

    pub fn toggle_task(&self, id: i32, finished: bool) -> Result<(), anyhow::Error> {
        DataManagement::toggle_task(id, finished)
    }

    pub fn del_task(&self, id: i32) -> Result<(), anyhow::Error> {
        DataManagement::del_task(id)
    }

    pub fn query_today_review(&self) -> Result<Vec<Task>, anyhow::Error> {
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
            "SELECT r.id, r.content, r.create_date, COALESCE(t.finished, false) AS finished 
            FROM records as r 
            LEFT JOIN tasks t 
            ON t.id = r.id AND t.create_date = '{}'
            where r.create_date in ({})
            order by r.create_date, r.id asc",
            chrono::Local::now().date_naive(),
            condition
        );
        DataManagement::query(query_sql, params_from_iter(&filter), move |row| {
            trace!("{row:?}");
            Ok(Task {
                id: row.get_unwrap(0),
                content: row.get_unwrap::<_, String>(1).into(),
                create_date: row.get_unwrap::<_, String>(2).into(),
                finished: row.get_unwrap(3),
            })
        })
    }
}

#[cfg(test)]
mod test {
    use cargo_metadata::MetadataCommand;
    use directories::ProjectDirs;

    #[test]
    fn directories_test() -> anyhow::Result<()> {
        let metadata = MetadataCommand::new()
            .no_deps() // Exclude dependency information (optional)
            .exec()?;

        // Get the root package (current project)
        let root_package = metadata
            .root_package()
            .expect("can't find the root package'");
        if let Some(proj_dirs) = ProjectDirs::from_path(root_package.name.as_str().into()) {
            // macOS: ~/Library/Application Support/com.mycompany.myapp/
            // Windows: C:\Users\<Username>\AppData\Roaming\MyCompany\MyApp\
            let data_dir = proj_dirs.data_dir();
            println!("Data Directory: {:?}", data_dir);

            // macOS: ~/Library/Caches/com.mycompany.myapp/
            // Windows: C:\Users\<Username>\AppData\Local\MyCompany\MyApp\cache\
            let cache_dir = proj_dirs.cache_dir();
            println!("Cache Directory: {:?}", cache_dir);
        } else {
            eprintln!("Could not determine project directories.");
        }
        Ok(())
    }
}
