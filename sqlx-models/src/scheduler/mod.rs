use crate::prelude::*;
use std::sync::Mutex;
mod migration;
pub(crate) use migration::Schema;

pub mod table;

use migration::Migration;

pub use table::Table;

pub struct Scheduler(Mutex<Migration>);

impl Scheduler {
    pub(crate) fn new() -> Self {
        Self(Mutex::new(Migration::new()))
    }
    /// Allows tables to register themselves into the migration.
    /// The first table to register will wait for 250 milliseconds before
    /// generating the migration files.
    pub fn register(&self, table: Table) {
        let len;
        {
            let mut migration = self.0.lock().unwrap();
            len = migration.queue.len();

            migration.queue.insert(table)
        }

        if len == 0 {
            std::thread::sleep(time::Duration::from_millis(250));
            self.commit()
        }
    }

    fn commit(&self) {
        let mut migr = self.0.lock().unwrap();
        migr.migrate();
        let err = migr.result.as_ref().err();
        let err_msg = err.map(|err| format!("{}", err));
        let kind = err.map(Error::kind);
        

        let json = serde_json::json!({
            "success": &migr.success,
            "error": {
                "kind": err_msg,
                "message": kind
            },
        });
        println!("<SQLX-OUTPUT>{0}</SQLX-OUTPUT>", json);
    }
}
