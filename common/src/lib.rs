pub mod warp_request_filter {
    use warp::hyper::{Method as RequestMethod};
    use warp::Filter;
    use warp::http::HeaderMap;
    use warp::hyper::body::Bytes;
    use warp::path::FullPath;

    pub type ProxyQueryParameters = Option<String>;
    pub type ProxyHeaders = HeaderMap;
    pub type ProxyMethod = RequestMethod;
    pub type ProxyRequest = (ProxyUri, ProxyQueryParameters, ProxyMethod, ProxyHeaders, Bytes);
    pub type ProxyUri = FullPath;
    pub type ProxyRequestBodyAsString = (ProxyUri, ProxyQueryParameters, ProxyMethod, ProxyHeaders, String);


    pub fn query_params_filter() -> impl Filter<Extract=(ProxyQueryParameters, ), Error=std::convert::Infallible> + Clone {
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

    pub fn extract_request_data_filter_body_as_string() -> impl Filter<Extract=ProxyRequestBodyAsString, Error=warp::Rejection> + Clone {
        warp::path::full()
            .and(query_params_filter())
            .and(warp::method())
            .and(warp::header::headers_cloned())
            .and(string_filter(10000))
    }

    // https://github.com/seanmonstar/warp/issues/248

    /// Extracts the body of a request as string
    pub fn string_filter(
        limit: u64,
    ) -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(limit)
            .and(warp::filters::body::bytes())
            .and_then(convert_to_string)
    }

    async fn convert_to_string(bytes: Bytes) -> Result<String, warp::Rejection> {
        String::from_utf8(bytes.to_vec())
            .map_err(|_| warp::reject())
    }

}

