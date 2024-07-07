use rusqlite::Connection;

pub struct DBManager {
    connection: Connection,
}

pub struct ConnectionError {}

impl DBManager {
    pub fn new() -> Result<DBManager, ConnectionError> {
        if let Ok(connection) = Connection::open("jdiff.db") {
            connection
                .pragma_update_and_check(None, "journal_mode", &"WAL", |_| Ok(()))
                .unwrap();
            return Ok(DBManager { connection });
        }

        Err(ConnectionError {})
    }

    pub fn get_connection(&self) -> &Connection {
        &self.connection
    }
    pub fn migrate_schema(self) {}
}
