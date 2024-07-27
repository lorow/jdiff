use std::fmt::Display;

use rusqlite::Connection;
use tracing::error;

use super::schema::{EditorContent, Request};

#[derive(Debug)]
pub enum DaoError {
    TooManyRowsReturned,
    QueryReturnedNoRows,
}

impl Display for DaoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DaoError::TooManyRowsReturned => write!(f, "Too many rows returned"),
            DaoError::QueryReturnedNoRows => write!(f, "Query returned no rows"),
        }
    }
}

impl std::error::Error for DaoError {}

pub struct RequestDao<'a> {
    conn: &'a Connection,
}

impl<'a> RequestDao<'a> {
    fn new(conn: &Connection) -> RequestDao {
        RequestDao { conn }
    }

    fn get_requests(&self) -> Result<Vec<Request>, Box<dyn std::error::Error>> {
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
                    headers: row.get(6)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(result)
    }
}

pub struct EditorContentDao<'a> {
    conn: &'a Connection,
}

impl<'a> EditorContentDao<'a> {
    fn new(conn: &Connection) -> EditorContentDao {
        EditorContentDao { conn }
    }

    fn get_editor_content(
        &self,
        project_id: i32,
    ) -> Result<EditorContent, Box<dyn std::error::Error>> {
        let mut statement = self.conn.prepare(
            "SELECT id, project_id, content from jdiff_editor_content where project_id = ?1",
        )?;

        let result = statement
            .query_map([project_id], |row| {
                Ok(EditorContent {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    content: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        if result.len() > 1 {
            error!(
                "Found multiple editor content rows for project {}, found {}",
                project_id,
                result.len()
            );
            return Err(Box::new(DaoError::TooManyRowsReturned));
        }

        if result.is_empty() {
            return Err(Box::new(DaoError::QueryReturnedNoRows));
        }

        Ok(result[0].clone())
    }
}
