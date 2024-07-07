#[derive(Debug)]

pub struct Request {
    id: i32,
    project_id: i32,
    name: String,
    body: String,
    url: String,
    additional_data: String,
    headsers: String,
}

pub struct EditorContent {
    id: i32,
    project_id: i32,
    content: String,
}

pub struct Project {
    id: i32,
    name: String,
}
