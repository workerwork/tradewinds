use std::time::Instant;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use tracing::info;

pub async fn request_logging_middleware(request: Request<Body>, next: Next) -> Result<Response, (StatusCode, String)> {
    let path = request.uri().path().to_owned();
    let method = request.method().clone();
    let start = Instant::now();

    info!("请求开始: {} {}", method, path);

    let response = next.run(request).await;
    let duration = start.elapsed();

    info!("请求完成: {} {} {} in {:?}", method, path, response.status(), duration);

    Ok(response)
}
