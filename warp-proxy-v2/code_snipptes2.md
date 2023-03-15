
struct BodyStream {
body: Body,
}

pub struct ProxyError {
inner: BoxError,
}

// body.rs from warp
type BoxError = Box<dyn StdError + Send + Sync>;

// body.rs from warp
impl ProxyError {
pub(crate) fn new<E: Into<BoxError>>(err: E) -> ProxyError {
ProxyError { inner: err.into() }
}
}

// body.rs from warp
impl Stream for BodyStream {
type Item = Result<Bytes, ProxyError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let opt_item = ready!(Pin::new(&mut self.get_mut().body).poll_next(cx));

        match opt_item {
            None => Poll::Ready(None),
            Some(item) => {
                let stream_buf = item.map_err(ProxyError::new);

                Poll::Ready(Some(stream_buf))
            }
        }
    }
}

// https://github.com/seanmonstar/warp/issues/448

// let body = body.map_ok(|mut buf| {
// buf.to_bytes()
// });

// https://github.com/seanmonstar/warp/issues/139
// fn extract_request() -> impl Filter<Extract=(http::Request<BodyStream>, ), Error=warp::Rejection> + Copy {
//     warp::method()
//         .and(warp::path::full())
//         .and(warp::header::headers_cloned())
//         .and(warp::body::stream())
//         .map(|method: http::Method, path: warp::path::FullPath, headers: http::HeaderMap, body: BodyStream| {
//             let mut req = http::Request::builder()
//                 .method(method.clone())
//                 .uri(path.as_str())
//                 .body(body)
//                 .expect("request builder");
//             {
//                 *req.headers_mut() = headers;
//             }
//
//
//             req
//         })
// }


