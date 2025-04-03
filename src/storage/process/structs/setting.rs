use std::fmt::Debug;
use bincode::{Decode, Encode};
use rusqlite::Row;

#[derive(Debug)]
pub struct Setting<T: Encode + Decode<()> + Debug> {
    pub key: String,
    pub value: Option<T>,
    pub enabled: bool,
}

impl <T: Encode + Decode<()> + Debug> From<&Row<'_>> for Setting<T> {
    fn from(row: &Row) -> Self {
        let value_bytes: Vec<u8>  = row.get(1).unwrap_or(vec![]);
        let mut value: Option<T> = None;

        if value_bytes.len() > 0 {
            let (val, _bytes) = bincode::decode_from_slice(&value_bytes, bincode::config::standard()).unwrap();
            value = Some(val);
        }

        Self {
            key: row.get(0).unwrap(),
            value,
            enabled: row.get(2).unwrap()
        }
    }
}
