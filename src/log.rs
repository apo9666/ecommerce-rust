use actix_web::HttpRequest;
use slog::{error, o, Drain, Logger};
use slog_async::Async;
use slog_envlogger;
use slog_term::{FullFormat, TermDecorator};
use uuid::Uuid;

use crate::error::AppError;

pub fn configure_log() -> Logger {
    let decorator = TermDecorator::new().build();
    let console_drain = FullFormat::new(decorator).build().fuse();
    let console_drain = slog_envlogger::new(console_drain);
    let console_drain = Async::new(console_drain).build().fuse();
    slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
}
pub fn log_error(log: Logger) -> impl Fn(AppError) -> AppError {
    move |err| {
        let log = log.new(o!(
            "cause" => err.cause.clone()
        ));
        error!(log, "{}", err.message());
        err
    }
}

pub fn get_header(req: &HttpRequest, name: &str) -> String {
    let Some(header_value) = req.headers().get(name) else {
        return "-".to_string();
    };

    let Ok(value) = header_value.to_str() else {
        return "-".to_string();
    };

    return value.to_string();
}

pub fn create_log(log: &Logger, req: &HttpRequest) -> Logger {
    let method = req.method().as_str().to_string();
    let url = req.uri().to_string();
    let user_agent = get_header(req, "user-agent");

    log.new(o!("method" => method, "url" => url, "user_agent" => user_agent, "request_id" => Uuid::new_v4().to_string()))
}
