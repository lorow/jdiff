pub trait ToSql {
    fn to_sql(&self) -> String;
}

pub struct Migration {
    pub name: String,
    pub sql: String,
}

impl ToSql for Migration {
    fn to_sql(&self) -> String {
        self.sql.clone()
    }
}

pub fn get_migrations() -> Vec<Migration> {
    Vec::from([
        Migration {
            name: "Create table jdiff_requests".to_string(),
            sql: "CREATE TABLE IF NOT EXISTS jdiff_requests (
                id INTEGER PRIMARY KEY,
                project_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                body TEXT NOT NULL,
                url TEST NOT NULL, 
                additional_data TEXT NOT NULL,
                headers TEXT NOT NULL,
            )"
            .to_string(),
        },
        Migration {
            name: "Create table jdiff_editor_content".to_string(),
            sql: "CREATE TABLE IF NOT EXISTS jdiff_editor_content (
                id INTEGER PRIMARY KEY,
                project_id INTEGER NOT NULL,
                content TEXT NOT NULL
            )"
            .to_string(),
        },
        Migration {
            name: "Create table jdiff_projects".to_string(),
            sql: "CREATE TABLE IF NOT EXISTS jdiff_projects (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            )"
            .to_string(),
        },
    ])
}
