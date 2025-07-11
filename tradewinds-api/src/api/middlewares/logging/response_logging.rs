use std::pin::Pin;

use axum::{
    body::Body,
    http::{Request, Response},
    middleware::Next,
};
use bytes::Bytes;
use http_body::Frame;
use pin_project_lite::pin_project;
use tracing::info;

pin_project! {
    pub struct LoggedBody<B> {
        #[pin]
        pub inner: B,
    }
}

impl http_body::Body for LoggedBody<Body> {
    type Data = Bytes;
    type Error = axum::Error;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let this = self.project();
        let poll = this.inner.poll_frame(cx);

        if let std::task::Poll::Ready(Some(Ok(frame))) = &poll {
            if let Some(data) = frame.data_ref() {
                if let Ok(body_str) = String::from_utf8(data.to_vec()) {
                    info!("响应体: {}", body_str);
                }
            }
        }

        poll
    }
}

pub async fn response_logging_middleware(request: Request<Body>, next: Next) -> Response<LoggedBody<Body>> {
    let response = next.run(request).await;
    let (parts, body) = response.into_parts();
    let logged_body = LoggedBody { inner: body };
    Response::from_parts(parts, logged_body)
}
