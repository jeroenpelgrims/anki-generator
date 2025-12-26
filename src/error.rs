use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub struct AppError(pub anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal error: {}", self.0),
        )
            .into_response()
    }
}
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self(err)
    }
}
