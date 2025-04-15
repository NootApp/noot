use std::fmt::Debug;
use bincode::{Decode, Encode};
use rusqlite::{Connection, Row};

#[derive(Debug)]
pub struct Setting<V: Encode + Decode<()> + Debug> {
    pub key: String,
    pub value: V
}

impl <V:Encode + Decode<()> + Debug> Setting<V> {
    pub fn new<K: Into<String>>(key: K, value: V) -> Self {
        Self {
            key: key.into(),
            value,
        }
    }

    pub fn store(&self, conn: &mut Connection) -> rusqlite::Result<usize> {
        conn.execute("INSERT INTO settings (id, value) VALUES (?, ?)", (&self.key, bincode::encode_to_vec(&self.value, bincode::config::standard()).unwrap()))
    }
}

impl <V: Encode + Decode<()> + Debug> From<&Row<'_>> for Setting<V> {
    fn from(row: &Row) -> Self {
        let value_bytes: Vec<u8>  = row.get(1).unwrap_or(vec![]);
        let value: V;

        if value_bytes.len() > 0 {
            let (val, _bytes) = bincode::decode_from_slice(&value_bytes, bincode::config::standard()).unwrap();
            value = val;
        } else {
            panic!("Database corruption detected")
        }

        Self {
            key: row.get(0).unwrap(),
            value,
        }
    }
}