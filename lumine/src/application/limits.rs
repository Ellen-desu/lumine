#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub max_uri_size: usize,
    pub max_query_size: usize,
    pub max_query_count: usize,
    pub max_headers_size: usize,
    pub max_headers_count: usize,
    pub max_body_size: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_uri_size: 8 * 1024, // 8 KB

            // Maximum query size is 800 KB
            max_query_size: 8 * 1024, // 8 KB
            max_query_count: 100,

            // Maximum headers size is 6.4 MB
            max_headers_size: 64 * 1024, // 64 KB
            max_headers_count: 100,

            max_body_size: 10 * 1024 * 1024, // 10 MB
        }
    }
}
