use cxx_qt::QObject;
use qt6_core::{QString, QStringList};
use rsqlite::{params, Connection};
use uuid::Uuid;

use crate::app_entry::AppEntry;

#[derive(QObject)]
pub struct AppDb {
    #[qproperty]
    pub uuid: QString,

    #[qproperty]
    pub path: QString,

    #[qproperty]
    pub entries: Vec<crate::app_entry::AppEntry>,

    #[qproperty(read, notify = "appsChanged")]
    apps: Vec<*mut AppEntry>,

    conn: Option<Connection>,
}

impl Default for AppDb {
    fn default() -> Self {
        AppDb {
            uuid: QString::from(Uuid::new_v4().to_string()),
            path: QString::default(),
            entries: vec![],
            apps: vec![],
            conn: None,
        }
    }
}

impl AppDb {
    #[qinvokable]
    pub fn setPath(&mut self, new_path: &QString) {
        if &self.path == new_path {
            return;
        }
        self.path = new_path.clone();
        self.conn = None;
        let db_path = if new_path.is_empty() {
            ":memory:".to_string()
        } else {
            new_path.to_string()
        };
        let conn = Connection::open(db_path).expect("Failed to open database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS frequencies (id TEXT PRIMARY KEY, frequency INTEGER)",
            [],
        )
        .unwrap();
        self.conn = Some(conn);
        self.update_app_frequencies();
    }

    #[qinvokable]
    pub fn setEntries(&mut self, new_entries: Vec<AppEntry>) {
        if self.entries == new_entries {
            return;
        }
        self.entries = new_entries;
        self.update_apps();
    }

    pub fn get_apps(&self) -> Vec<&mut AppEntry> {
        let mut apps: Vec<_> = self.apps.clone();
        apps.sort_by(|a, b| unsafe {
            let fa = (**a).frequency;
            let fb = (**b).frequency;
            if fa != fb {
                fb.cmp(&fa)
            } else {
                (**a).name.cmp(&(**b).name)
            }
        });
        apps
    }

    #[qinvokable]
    pub fn incrementFrequency(&mut self, id: &QString) {
        if let Some(conn) = &self.conn {
            conn.execute(
                "INSERT INTO frequencies (id, frequency) VALUES (?1, 1)
                ON CONFLICT(id) DO UPDATE SET frequency = frequency + 1",
                params![id.to_string()],
            )
            .unwrap();
        }
        for entry in &mut self.entries {
            if entry.id == *id {
                entry.frequency += 1;
                break;
            }
        }
        self.update_apps();
    }

    fn update_app_frequencies(&mut self) {
        if let Some(conn) = &self.conn {
            for entry in &mut self.entries {
                let freq: u32 = conn
                    .query_row(
                        "SELECT frequency FROM frequencies WHERE id = ?1",
                        params![entry.id.to_string()],
                        |row| row.get(0),
                    )
                    .unwrap_or(0);
                entry.frequency = freq;
            }
        }
    }

    fn update_apps(&mut self) {
        self.update_app_frequencies();
        let mut new_apps: Vec<*mut AppEntry> = self
            .entries
            .iter_mut()
            .map(|e| e as *mut AppEntry)
            .collect();
        new_apps.sort_by(|a, b| unsafe {
            let fa = (**a).frequency;
            let fb = (**b).frequency;
            if fa != fb {
                fb.cmp(&fa)
            } else {
                (**a).name.cmp(&(**b).name)
            }
        });
        if new_apps != self.apps {
            self.apps = new_apps;
            self.appsChanged();
        }
    }

    #[cxx_qt::qsignal]
    fn appsChanged(&self);
}

pub fn register() {
    qml_register_type::<AppDb>("Vela", 1, 0, "AppDb");
}
