use log::debug;
use rusqlite::Connection;

use crate::component::data_management::DB_FILE;

pub struct DbChecker;

const RECORD_TABLE_SQL: &str = "CREATE TABLE IF NOT EXISTS records (
  id INTEGER PRIMARY KEY,
  content TEXT,
  create_date DATE
)";

const IDX_RECORD_DATE: &str = "CREATE INDEX IF NOT EXISTS idx_record_create_date ON records (create_date)";

impl DbChecker {
    pub fn create_check_db() {
        let con = DB_FILE
            .with(|p| Connection::open(p))
            .unwrap_or_else(|e| panic!("Failed to open the database file: {e:?}"));

        // create record table
        debug!("Create records table: {RECORD_TABLE_SQL}");
        con.execute(RECORD_TABLE_SQL, [])
            .unwrap_or_else(|msg| panic!("Fail to create the initialized table, error: {msg}, sql: {RECORD_TABLE_SQL}"));

        debug!("Create inddex: {IDX_RECORD_DATE}");
        con.execute(IDX_RECORD_DATE, [])
            .unwrap_or_else(|msg| panic!("Fail to create index, error: {msg}, sql: {IDX_RECORD_DATE}"));

        // add in version 0.3.1
        let checked_table = "records";
        let checked_field = "checkpoint_date";
        let exists: bool = con
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info(?) WHERE name = ?",
                [checked_table, checked_field],
                |row| row.get::<_, i64>(0).map(|v| v > 0),
            )
            .unwrap_or(false);
        if !exists {
            let sql = format!("ALTER TABLE {checked_table} ADD COLUMN {checked_field} DE");
            con.execute(&sql, []).expect("Add new column {checked_field} failed.");
            let sql = format!("UPDATE {checked_table} set {checked_field} = create_date WHERE {checked_field} is NULL");
            con.execute(&sql, []).expect("Update new column {checked_field} failed.");
            debug!("Added column '{checked_field}' to table '{checked_table}'");
        } else {
            debug!("Column '{checked_field}' already exists in table '{checked_table}'");
        }

        const TASK_TABLE_SQL: &str = "CREATE TABLE IF NOT EXISTS tasks (
          id INTEGER NOT NULL,
          create_date DATE NOT NULL,
          finished BOOLEAN NOT NULL,
          PRIMARY KEY (id, create_date)
        )";
        debug!("Create task table: {TASK_TABLE_SQL}");
        con.execute(TASK_TABLE_SQL, [])
            .unwrap_or_else(|msg| panic!("Fail to create the initialized table, error: {msg}, sql: {TASK_TABLE_SQL}"));
    }
}
