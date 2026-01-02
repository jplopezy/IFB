//! Comprehensive cURL Knowledge Base
//! Contains all URL schemes, protocols, headers, and options for LLM context

/// Common HTTP Request Headers (RFC 7231, 7232, 7234, 7235)
pub const HTTP_REQUEST_HEADERS: &[&str] = &[
    // Standard headers
    "Accept",
    "Accept-Charset",
    "Accept-Encoding",
    "Accept-Language",
    "Authorization",
    "Cache-Control",
    "Connection",
    "Content-Length",
    "Content-Type",
    "Cookie",
    "Date",
    "Expect",
    "From",
    "Host",
    "If-Match",
    "If-Modified-Since",
    "If-None-Match",
    "If-Range",
    "If-Unmodified-Since",
    "Max-Forwards",
    "Origin",
    "Pragma",
    "Proxy-Authorization",
    "Range",
    "Referer",
    "TE",
    "Transfer-Encoding",
    "Upgrade",
    "User-Agent",
    "Via",
    "Warning",
    // Common non-standard headers
    "X-Forwarded-For",
    "X-Forwarded-Proto",
    "X-Real-IP",
    "X-Requested-With",
    "X-Custom-Header",
    // Security headers
    "X-Frame-Options",
    "X-Content-Type-Options",
    "X-XSS-Protection",
    // Malformed for fuzzing
    "",
    " ",
    "\x00",
    "\n",
    "\r\n",
];

/// Common HTTP Response Headers
pub const HTTP_RESPONSE_HEADERS: &[&str] = &[
    "Accept-Ranges",
    "Age",
    "Allow",
    "Cache-Control",
    "Connection",
    "Content-Encoding",
    "Content-Language",
    "Content-Length",
    "Content-Location",
    "Content-Range",
    "Content-Type",
    "Date",
    "ETag",
    "Expires",
    "Last-Modified",
    "Location",
    "Proxy-Authenticate",
    "Public-Key-Pins",
    "Retry-After",
    "Server",
    "Set-Cookie",
    "Strict-Transport-Security", // HSTS
    "Transfer-Encoding",
    "Upgrade",
    "Vary",
    "Via",
    "Warning",
    "WWW-Authenticate",
    // Alt-Svc (RFC 7838)
    "Alt-Svc",
    // Security headers
    "X-Frame-Options",
    "X-Content-Type-Options",
    "X-XSS-Protection",
    "Content-Security-Policy",
    // Malformed for fuzzing
    "",
    " ",
    "\x00",
    "\n",
    "\r\n",
];

/// Common HTTP Methods
pub const HTTP_METHODS: &[&str] = &[
    "GET", "POST", "PUT", "DELETE", "HEAD", "OPTIONS", "PATCH", "TRACE", "CONNECT",
    // Invalid for fuzzing
    "", "INVALID",
];

/// Common URL Path Patterns for Fuzzing
pub const URL_PATHS: &[&str] = &[
    "/",
    "/path",
    "/path/to/resource",
    "/../",
    "/./",
    "/..",
    "/.",
    "/%00",
    "/%FF",
    "/%2F",
    "/%2E",
    "/%2E%2E",
    "/%2E%2E%2F",
    "/path/../",
    "/path/./",
    "/path/%00",
    "/path?query",
    "/path#fragment",
    "/path/with/many/segments",
    "/%C0%AF",    // UTF-8 overlong encoding
    "/%E0%80%AF", // UTF-8 overlong
    "//",
    "///",
    "/path//",
    "/path///",
];

/// Common URL Query Patterns
pub const URL_QUERIES: &[&str] = &[
    "?",
    "?key=value",
    "?key=value&foo=bar",
    "?%00",
    "?%FF",
    "?key=%00",
    "?key=%FF",
    "?key=value%00",
    "?&",
    "?&&",
    "?key=",
    "?=value",
    "?key=value&",
    "?key=value&&",
];

/// Common URL Fragment Patterns
pub const URL_FRAGMENTS: &[&str] = &["#", "#section", "#%00", "#%FF", "#section%00"];

/// Common Host Patterns for Fuzzing
pub const URL_HOSTS: &[&str] = &[
    "localhost",
    "127.0.0.1",
    "0.0.0.0",
    "255.255.255.255",
    "example.com",
    "test.com",
    "subdomain.example.com",
    "[::1]",         // IPv6
    "[::]",          // IPv6
    "[2001:db8::1]", // IPv6
    "localhost.localdomain",
    ".",  // Invalid
    "..", // Invalid
    "",   // Invalid
];

/// Common Port Numbers
pub const URL_PORTS: &[u16] = &[
    80,    // HTTP
    443,   // HTTPS
    21,    // FTP
    22,    // SSH/SFTP
    25,    // SMTP
    110,   // POP3
    143,   // IMAP
    993,   // IMAPS
    995,   // POP3S
    8080,  // HTTP alternate
    8443,  // HTTPS alternate
    8888,  // Common test port
    0,     // Invalid
    65535, // Max
];

/// All protocols supported by libcurl (complete list)
pub const PROTOCOLS: &[&str] = &[
    "HTTP",
    "HTTPS",
    "FTP",
    "FTPS",
    "SFTP",
    "SCP",
    "IMAP",
    "IMAPS",
    "POP3",
    "POP3S",
    "SMTP",
    "SMTPS",
    "LDAP",
    "LDAPS",
    "DICT",
    "GOPHER",
    "GOPHERS",
    "TELNET",
    "TFTP",
    "RTSP",
    "RTMP",
    "RTMPE",
    "RTMPS",
    "RTMPT",
    "RTMPTE",
    "WebSocket",
    "WebSocket Secure",
    "FILE",
    "MQTT",
    "SMB",
    "SMBS",
];

/// All URL schemes (expanded from protocols)
pub const URL_SCHEMES: &[&str] = &[
    // HTTP protocols
    "http://",
    "https://",
    // File protocols
    "file://",
    // FTP protocols
    "ftp://",
    "ftps://",
    // Email protocols
    "imap://",
    "imaps://",
    "pop3://",
    "pop3s://",
    "smtp://",
    "smtps://",
    // Directory protocols
    "ldap://",
    "ldaps://",
    // Other protocols
    "dict://",
    "gopher://",
    "gophers://",
    "telnet://",
    "tftp://",
    "rtsp://",
    "rtmp://",
    "rtmpe://",
    "rtmps://",
    "rtmpt://",
    "rtmpte://",
    "scp://",
    "sftp://",
    "smb://",
    "smbs://",
    "mqtt://",
    "mqtts://",
    // WebSocket
    "ws://",
    "wss://",
    // Invalid/malformed for fuzzing
    "://",
    "",
    "http:/",
    "https:/",
    "http:",
    "https:",
];

/// Common libcurl Options (CURLOPT_*) for fuzzing
pub const CURL_OPTIONS: &[&str] = &[
    // URL and connection
    "CURLOPT_URL",
    "CURLOPT_TIMEOUT",
    "CURLOPT_CONNECTTIMEOUT",
    // HTTP specific
    "CURLOPT_HTTPHEADER",
    "CURLOPT_USERAGENT",
    "CURLOPT_COOKIE",
    "CURLOPT_COOKIEFILE",
    "CURLOPT_COOKIEJAR",
    "CURLOPT_HTTPGET",
    "CURLOPT_POST",
    "CURLOPT_POSTFIELDS",
    "CURLOPT_HTTPPOST",
    "CURLOPT_MIMEPOST",
    "CURLOPT_PUT",
    "CURLOPT_CUSTOMREQUEST",
    "CURLOPT_HTTP_VERSION",
    "CURLOPT_FOLLOWLOCATION",
    "CURLOPT_MAXREDIRS",
    // Authentication
    "CURLOPT_USERPWD",
    "CURLOPT_HTTPAUTH",
    "CURLOPT_USERNAME",
    "CURLOPT_PASSWORD",
    "CURLOPT_XOAUTH2_BEARER",
    "CURLOPT_LOGIN_OPTIONS",
    // Security
    "CURLOPT_SSL_VERIFYPEER",
    "CURLOPT_SSL_VERIFYHOST",
    "CURLOPT_CAINFO",
    "CURLOPT_CAPATH",
    "CURLOPT_CRLFILE",
    "CURLOPT_SSH_HOST_PUBLIC_KEY_SHA256",
    "CURLOPT_HSTS",
    "CURLOPT_HSTS_CTRL",
    // Headers
    "CURLOPT_HEADER",
    "CURLOPT_HEADERFUNCTION",
    "CURLOPT_HEADERDATA",
    // Callbacks
    "CURLOPT_WRITEFUNCTION",
    "CURLOPT_READFUNCTION",
    "CURLOPT_PROGRESSFUNCTION",
];

/// RFCs relevant to cURL fuzzing
pub const RELEVANT_RFCS: &[&str] = &[
    "RFC 7230: HTTP/1.1 Message Syntax and Routing",
    "RFC 7231: HTTP/1.1 Semantics and Content",
    "RFC 7232: HTTP/1.1 Conditional Requests",
    "RFC 7233: HTTP/1.1 Range Requests",
    "RFC 7234: HTTP/1.1 Caching",
    "RFC 7235: HTTP/1.1 Authentication",
    "RFC 3986: URI Generic Syntax",
    "RFC 2616: HTTP/1.1 (obsolete but still referenced)",
    "RFC 7540: HTTP/2",
    "RFC 7838: HTTP Alternative Services (Alt-Svc)",
    "RFC 6797: HTTP Strict Transport Security (HSTS)",
    "RFC 6265: HTTP State Management Mechanism (Cookies)",
    "RFC 2818: HTTP Over TLS",
    "RFC 2817: Upgrading to TLS Within HTTP/1.1",
];

/// Generate comprehensive context for LLM
pub fn get_curl_knowledge_context() -> String {
    format!(
        r#"# cURL Fuzzing Knowledge Base

## Supported URL Schemes:
{}

## HTTP Request Headers:
{}

## HTTP Response Headers:
{}

## HTTP Methods:
{}

## Protocols Supported:
{}

## Common libcurl Options:
{}

## Relevant RFCs:
{}

## Fuzzing Patterns:
- URL Paths: {}
- URL Queries: {}
- URL Fragments: {}
- URL Hosts: {}
- URL Ports: {}

Use this knowledge to generate intelligent mutations that explore edge cases and potential vulnerabilities in cURL's URL parsing and protocol handling."#,
        URL_SCHEMES.join(", "),
        HTTP_REQUEST_HEADERS.join(", "),
        HTTP_RESPONSE_HEADERS.join(", "),
        HTTP_METHODS.join(", "),
        PROTOCOLS.join(", "),
        CURL_OPTIONS.join(", "),
        RELEVANT_RFCS.join(", "),
        URL_PATHS.join(", "),
        URL_QUERIES.join(", "),
        URL_FRAGMENTS.join(", "),
        URL_HOSTS.join(", "),
        URL_PORTS
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    )
}
