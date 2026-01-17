use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            message: String::from("success"),
            data: Some(data),
        }
    }

    pub fn error(message: String) -> Self {
        ApiResponse {
            message,
            data: None
        }
    }
}
