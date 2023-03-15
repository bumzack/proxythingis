// pub fn http_request() -> impl Filter<Extract = (http::Request<Bytes>,), Error = Rejection> + Copy {
// // TODO: extract `hyper::Request` instead
// // blocked by https://github.com/seanmonstar/warp/issues/139
// warp::any()
// .and(warp::method())
// .and(warp::filters::path::full())
// .and(warp::filters::query::raw())
// .and(warp::header::headers_cloned())
// .and(warp::body::bytes())
// .and_then(|method, path: FullPath, query, headers, bytes| async move {
// let uri = http::uri::Builder::new()
// .path_and_query(format!("{}?{}", path.as_str(), query))
// .build()
// .map_err(Error::from)?;
//
// let mut request = http::Request::builder()
// .method(method)
// .uri(uri)
// .body(bytes)
// .map_err(Error::from)?;
//
//             *request.headers_mut() = headers;
//
// Ok::<http::Request<Bytes>, Rejection>(request)
// })
// }
//
// #[derive(  Debug)]
// pub enum Error {
// #[error(transparent)]
// Http(#[from] http::Error),
// }









