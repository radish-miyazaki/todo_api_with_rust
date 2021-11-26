use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AddTodoRequest {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    pub id: u32,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteTodoRequest {
    pub id: u32,
}
