use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TodoResponse {
    pub id: u32,
    pub text: String,
}
