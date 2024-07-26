use rusqlite::Connection;

use super::schema::Request;

struct RequestDao<'a> {
    conn: &'a Connection,
}

impl<'a> RequestDao<'a> {
    fn new(conn: &Connection) -> RequestDao {
        RequestDao { conn }
    }

    fn get_requests(&self) -> Result<Vec<Request>, rusqlite::Error> {
        let mut statement = self.conn.prepare(
            "SELECT id, project_id, name, body, url, additional_data, headers from jdiff_requests",
        )?;

        let result = statement
            .query_map([], |row| {
                Ok(Request {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    name: row.get(2)?,
                    body: row.get(3)?,
                    url: row.get(4)?,
                    additional_data: row.get(5)?,
                    headsers: row.get(6)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(result)
    }
}
