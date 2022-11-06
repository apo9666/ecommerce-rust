use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use core::fmt;
use serde::Serialize;

#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    DbValidationError,
    ValidationError,
    NotFoundError,
}

#[derive(Debug)]
pub struct AppError {
    pub cause: Option<String>,
    pub message: Option<String>,
    pub error_type: AppErrorType,
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl AppError {
    // we are handling the none. function name should match field name
    fn message(&self) -> String {
        match &*self {
            // Error message is found then clone otherwise default message
            AppError {
                cause: _,
                message: Some(message),
                error_type: _,
            } => message.clone(),
            AppError {
                cause: _,
                message: None,
                error_type: AppErrorType::NotFoundError,
            } => "The requested item was not found".to_string(),
            AppError {
                cause: Some(cause),
                message: None,
                error_type: AppErrorType::ValidationError,
            } => cause.clone(),
            _ => "An unexpected error has occured".to_string(),
        }
    }
    // This db_error is used when we haven't implmented the From trait

    // pub fn db_error(error: impl ToString) -> AppError {
    //     AppError {
    //         cause: Some(error.to_string()),
    //         message: None,
    //         error_type: AppErrorType::DbError,
    //     }
    // }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for AppError {
    //error_response and status_code are the provided methods for ResponseError Trait

    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::DbValidationError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
            AppErrorType::ValidationError => StatusCode::LENGTH_REQUIRED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}
