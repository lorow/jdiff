use rusqlite::Connection;

pub trait SchemaModel {}

pub struct Request {
    id: i32,
    project_id: i32,
    name: String,
    body: String,
    url: String,
    additional_data: String,
    headsers: String,
}

impl SchemaModel for Request {}

pub struct EditorContent {
    id: i32,
    project_id: i32,
    content: String,
}

impl SchemaModel for EditorContent {}

pub struct Project {
    id: i32,
    name: String,
}

impl SchemaModel for Project {}
