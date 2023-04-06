pub mod warp_request_filter {
    use warp::{Buf, Filter, Stream};
    use warp::http::HeaderMap;
    use warp::hyper::body::Bytes;
    use warp::hyper::Method as RequestMethod;
    use warp::path::FullPath;

    pub type ProxyQueryParameters = Option<String>;
    pub type ProxyHeaders = HeaderMap;
    pub type ProxyMethod = RequestMethod;
    pub type ProxyRequest = (
        ProxyUri,
        ProxyQueryParameters,
        ProxyMethod,
        ProxyHeaders,
        Bytes,
    );
    pub type ProxyUri = FullPath;
    pub type ProxyRequestBodyAsString = (
        ProxyUri,
        ProxyQueryParameters,
        ProxyMethod,
        ProxyHeaders,
        String,
    );

    pub fn query_params_filter() -> impl Filter<Extract=(ProxyQueryParameters, ), Error=std::convert::Infallible> + Clone
    {
        warp::query::raw()
            .map(Some)
            .or_else(|_| async { Ok::<(ProxyQueryParameters, ), std::convert::Infallible>((None, )) })
    }

    pub fn extract_request_data_filter() -> impl Filter<Extract=ProxyRequest, Error=warp::Rejection> + Clone {
        warp::path::full()
            .and(query_params_filter())
            .and(warp::method())
            .and(warp::header::headers_cloned())
            .and(warp::body::bytes())
    }

    pub fn extract_request_data_filter_body_stream() -> impl Filter<
        Extract=(
            ProxyUri,
            ProxyQueryParameters,
            ProxyMethod,
            ProxyHeaders,
            impl Stream<Item=Result<impl Buf, warp::Error>> + Send + 'static,
        ),
        Error=warp::Rejection,
    > + Clone {
        warp::path::full()
            .and(query_params_filter())
            .and(warp::method())
            .and(warp::header::headers_cloned())
            .and(warp::body::stream())
    }

    pub fn extract_request_data_filter_body_as_string() -> impl Filter<Extract=ProxyRequestBodyAsString, Error=warp::Rejection> + Clone {
        warp::path::full()
            .and(query_params_filter())
            .and(warp::method())
            .and(warp::header::headers_cloned())
            .and(string_filter())
    }

    // https://github.com/seanmonstar/warp/issues/248

    /// Extracts the body of a request as string
    pub fn string_filter() -> impl Filter<Extract=(String, ), Error=warp::Rejection> + Clone {
        warp::filters::body::bytes().and_then(convert_to_string)
    }

    async fn convert_to_string(bytes: Bytes) -> Result<String, warp::Rejection> {
        match String::from_utf8(bytes.to_vec()) {
            Ok(b) => Ok(b),
            Err(_) => Ok(String::default()),
        }
    }
}
