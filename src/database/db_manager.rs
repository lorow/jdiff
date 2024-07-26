use rusqlite::Connection;
use tracing::info;

use super::migrations::get_migrations;

pub struct DBManager {
    connection: Connection,
}

pub struct ConnectionError {}

impl DBManager {
    pub fn new() -> Result<DBManager, ConnectionError> {
        if let Ok(connection) = Connection::open("jdiff.db") {
            connection
                .pragma_update_and_check(None, "journal_mode", "WAL", |_| Ok(()))
                .unwrap();
            return Ok(DBManager { connection });
        }

        Err(ConnectionError {})
    }

    pub fn get_connection(&self) -> &Connection {
        &self.connection
    }

    pub fn close(self) {
        self.connection.close().unwrap();
    }

    pub fn migrate_schema(self) {
        info!("Beginning applying migrations");
        let migrations = get_migrations();
        info!("migrations count {:?}", migrations.len());

        for migration in migrations {
            info!("Applying migration: {}", migration.name);
            migration.run(&self.connection);
        }
        info!("Migrations applied succesfully!");
    }
}
