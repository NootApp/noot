use std::fmt::Debug;
use bincode::{Decode, Encode};
use dark_light::Mode;
use rusqlite::Connection;
use rusqlite::fallible_streaming_iterator::FallibleStreamingIterator;
use crate::config::locate_config_dir;
use crate::storage::process::structs::setting::Setting;
use crate::storage::process::structs::workspace::Workspace;
use crate::utils::time::local_to_sqlstr;

pub mod structs;

const SEED_TABLES: &'static str = include_str!("../../../database/program.sql");
const SEED_DATA: &'static str = include_str!("../../../database/program.seed.sql");

#[derive(Debug)]
pub struct ProcessStorageManager {
    db: Connection,
}

impl ProcessStorageManager {
    pub fn new() -> ProcessStorageManager {
        let mut data_dir = locate_config_dir().unwrap();
        data_dir.push("noot.db");
        let db = Connection::open(&data_dir);

        let mut pm = if let Ok(db) = db {
            info!("Opened database from {}", data_dir.display());
            ProcessStorageManager { db }
        } else {
            warn!("Failed to open database, creating in memory store instead");
            ProcessStorageManager { db: Connection::open_in_memory().unwrap() }
        };

        let is_initialized: bool = pm.db.query_row("SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='workspaces');", (), |r| {
            r.get(0)
        }).unwrap();

        if !is_initialized {
            let tx = pm.db.transaction().unwrap();
            tx.execute_batch(SEED_TABLES).unwrap();
            tx.commit().unwrap();
        }

        let current_locale = rust_i18n::locale().to_string();

        // Extra settings which require runtime configuration....
        let settings = [
            ("runtime.daemon.enable", None, true),
            ("workspace.load_last", None, false),
            ("rpc.enabled", None, true),
            ("rpc.client_id", Some(bincode::encode_to_vec("", bincode::config::standard()).unwrap()), true),
            ("rpc.enable_idle", None, true),
            ("rpc.show_current_workspace", None, true),
            ("rpc.show_current_file", None, true),
            ("language.locale", Some(bincode::encode_to_vec(&current_locale, bincode::config::standard()).unwrap()), true),
            ("appearance.font.primary", Some(bincode::encode_to_vec("Roboto", bincode::config::standard()).unwrap()), true),
            ("appearance.font.monospace", Some(bincode::encode_to_vec("Roboto Mono", bincode::config::standard()).unwrap()), true),
            ("appearance.font.dyslexic.enable", None, false),
            ("appearance.font.dyslexic.primary", Some(bincode::encode_to_vec("OpenDyslexic", bincode::config::standard()).unwrap()), true),
            ("appearance.font.dyslexic.monospace", Some(bincode::encode_to_vec("OpenDyslexic Mono", bincode::config::standard()).unwrap()), true),
            ("appearance.theme.name", Some(bincode::encode_to_vec("Noot", bincode::config::standard()).unwrap()), true),
            ("appearance.theme.variant", Some(bincode::encode_to_vec(choose_day_night(), bincode::config::standard()).unwrap()), true),
            ("appearance.theme.adaptive_variance", None, false),
            ("appearance.theme.adaptive_variant_day", Some(bincode::encode_to_vec("light", bincode::config::standard()).unwrap()), false),
            ("appearance.tts.enable", None, true),
            ("appearance.tts.provider", Some(bincode::encode_to_vec("google", bincode::config::standard()).unwrap()), true)
        ];
        for setting in settings.iter() {
            pm.db.execute("INSERT OR IGNORE INTO settings (id, value, enabled) VALUES (?, ?, ?) ", (setting.0, setting.1.clone(), setting.2)).unwrap();
        }

        pm
    }

    pub fn list_workspaces(&self) -> Vec<Workspace> {
        let mut workspaces: Vec<Workspace> = Vec::new();

        let mut wksp_stmt = self.db.prepare("SELECT id, name, disk_path, last_accessed FROM workspaces ORDER BY last_accessed DESC").unwrap();

        wksp_stmt.query(()).unwrap().for_each(|row| {
            workspaces.push(structs::workspace::Workspace::from(row));
        }).unwrap();

        workspaces
    }

    pub(crate) fn create_workspace(&self, workspace: Workspace) {
        let mut statement = self.db.prepare("INSERT INTO workspaces (id, long_id, name, disk_path, last_accessed) VALUES (?, ?, ?, ?, ?) RETURNING *").unwrap();
        statement.query((workspace.id, workspace.long_id, workspace.name, workspace.disk_path, local_to_sqlstr(workspace.last_accessed))).unwrap().for_each(|row| {
            debug!("Created workspace row: {:?}", row);
        }).unwrap();
    }

    pub fn update_workspace(&self, id: impl Into<String>, ts: chrono::DateTime<chrono::Local>) {
        let mut statement = self.db.prepare("UPDATE workspaces SET last_accessed = ? WHERE id = ?").unwrap();
        statement.execute((local_to_sqlstr(ts), id.into())).unwrap();
    }

    pub fn get_setting<T: Encode + Decode<()> + Debug>(&self, key: impl Into<String>) -> Option<Setting<T>> {
        let k = key.into();
        let mut stmt = self.db.prepare("SELECT * FROM settings WHERE id = ?").unwrap();
        let outcome = stmt.query_row([&k], |r| Ok(Some(Setting::from(r))));

        if let Ok(outcome) = outcome {
            outcome
        } else {
            error!("Setting didnt exist: '{}'", k);
            None
        }
    }

    pub fn set_setting<T: Encode + Decode<()> + Debug>(&mut self, key: impl Into<String>, value: T, enabled: bool) {
        let k = key.into();
        let v = bincode::encode_to_vec(value, bincode::config::standard()).unwrap();
        let mut stmt = self.db.prepare("UPDATE settings SET value = ?, enabled = ? WHERE id = ?").unwrap();
        stmt.execute((v, enabled, k)).unwrap();
    }
}


fn choose_day_night() -> &'static str {
    match dark_light::detect().unwrap() {
        Mode::Dark => "dark",
        Mode::Light => "light",
        Mode::Unspecified => "light"
    }
}