use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures::future::{ok, LocalBoxFuture, Ready};
use futures::task::Poll;
use slog::info;
use slog::{o, Drain, Logger};
use slog_async::Async;
use slog_envlogger;
use slog_term::{FullFormat, TermDecorator};
use std::task::Context;

pub fn configure_log() -> Logger {
    let decorator = TermDecorator::new().build();
    let console_drain = FullFormat::new(decorator).build().fuse();
    let console_drain = slog_envlogger::new(console_drain);
    let console_drain = Async::new(console_drain).build().fuse();
    slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
}

// There are two step in middleware processing.
// 1. Middleware initialization, middleware factory get called with
//    next service in chain as parameter.
// 2. Middleware's call method get called with normal request.
pub struct Logging {
    logger: slog::Logger,
}

impl Logging {
    pub fn new(logger: slog::Logger) -> Logging {
        Logging { logger }
    }
}

// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Logging
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggingMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggingMiddleware {
            service,
            logger: self.logger.clone(),
        })
    }
}

pub struct LoggingMiddleware<S> {
    service: S,
    logger: slog::Logger,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        let logger = self.logger.clone();

        Box::pin(async move {
            let start_time = chrono::Utc::now();

            let res = fut.await?;
            let req = res.request();

            let end_time = chrono::Utc::now();
            let duration = end_time - start_time;

            info!(logger, "handled request";
                "responseTime" => duration.num_milliseconds(),
                "url" => %req.uri(),
                "route" => req.path(),
                "method" => %req.method(),
                "statusCode" => res.status().as_u16()
            );

            Ok(res)
        })
    }
}
