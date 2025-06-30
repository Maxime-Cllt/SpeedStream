use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    /// Creates a new ApiResponse with success status and data.
    pub fn success(message: &str, data: T) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: Some(data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_response_success() {
        let response: ApiResponse<String> =
            ApiResponse::success("Operation successful", "Data".into());
        assert!(response.success);
        assert_eq!(response.message, "Operation successful");
        assert_eq!(response.data, Some("Data".to_string()));
    }
}
