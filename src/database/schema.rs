pub struct Request {
    pub id: i32,
    pub project_id: i32,
    pub name: String,
    pub body: String,
    pub url: String,
    pub additional_data: String,
    pub headers: String,
}

pub struct EditorContent {
    pub id: i32,
    pub project_id: i32,
    pub content: String,
}

pub struct Project {
    pub id: i32,
    pub name: String,
}
