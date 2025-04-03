use std::fmt::Debug;
use chrono::{DateTime, Local};
use rusqlite::Row;
use crate::utils::time::sqlstr_to_local;

#[derive(Debug, Clone)]
pub struct Workspace {
    pub id: String,
    pub long_id: String,
    pub name: String,
    pub disk_path: String,
    pub last_accessed: DateTime<Local>,
}


impl From<&Row<'_>> for Workspace {
    fn from(row: &Row) -> Self {
        let raw_last_accessed: String = row.get(3).unwrap();
        let last_accessed = sqlstr_to_local(raw_last_accessed);

        Workspace {
            id: row.get(0).unwrap(),
            long_id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            disk_path: row.get(2).unwrap(),
            last_accessed,
        }
    }
}

