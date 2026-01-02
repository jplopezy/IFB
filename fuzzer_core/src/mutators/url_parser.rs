//! URL Parser for Structure-Aware Fuzzing
//! Parses URLs into components for intelligent mutation

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedURL {
    pub scheme: String,      // http, https, ftp
    pub host: String,        // example.com
    pub port: Option<u16>,   // 80, 443, 8080
    pub path: String,        // /path/to/resource
    pub query: String,       // ?key=value&foo=bar
    pub fragment: String,    // #section
    pub userinfo: String,    // user:pass@
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum URLComponent {
    Scheme,
    Host,
    Port,
    Path,
    Query,
    Fragment,
    UserInfo,
}

impl ParsedURL {
    /// Parse a URL string into components
    pub fn parse(url: &str) -> Option<Self> {
        let mut scheme = String::new();
        let mut host = String::new();
        let mut port = None;
        #[allow(unused_assignments)]
        let mut path = String::new();
        let mut query = String::new();
        let mut fragment = String::new();
        let mut userinfo = String::new();

        // Find scheme
        if let Some(scheme_end) = url.find("://") {
            scheme = url[..scheme_end].to_string();
            let after_scheme = &url[scheme_end + 3..];
            
            // Find userinfo (user:pass@)
            let (auth_part, rest) = if let Some(at_pos) = after_scheme.find('@') {
                userinfo = after_scheme[..at_pos].to_string();
                (&after_scheme[at_pos + 1..], &after_scheme[at_pos + 1..])
            } else {
                (after_scheme, after_scheme)
            };
            
            // Find host and port
            let (host_part, path_part) = if let Some(slash_pos) = auth_part.find('/') {
                (&auth_part[..slash_pos], &auth_part[slash_pos..])
            } else if let Some(query_pos) = auth_part.find('?') {
                (&auth_part[..query_pos], "")
            } else if let Some(frag_pos) = auth_part.find('#') {
                (&auth_part[..frag_pos], "")
            } else {
                (auth_part, "")
            };
            
            // Parse host:port
            if let Some(colon_pos) = host_part.rfind(':') {
                // Check if it's a port (not IPv6)
                if !host_part.contains('[') {
                    if let Ok(p) = host_part[colon_pos + 1..].parse::<u16>() {
                        host = host_part[..colon_pos].to_string();
                        port = Some(p);
                    } else {
                        host = host_part.to_string();
                    }
                } else {
                    host = host_part.to_string();
                }
            } else {
                host = host_part.to_string();
            }
            
            // Parse path, query, fragment
            let full_path = if path_part.is_empty() {
                if let Some(query_pos) = rest.find('?') {
                    &rest[query_pos..]
                } else if let Some(frag_pos) = rest.find('#') {
                    &rest[frag_pos..]
                } else {
                    ""
                }
            } else {
                path_part
            };
            
            if let Some(query_pos) = full_path.find('?') {
                path = full_path[..query_pos].to_string();
                let after_query = &full_path[query_pos + 1..];
                if let Some(frag_pos) = after_query.find('#') {
                    query = after_query[..frag_pos].to_string();
                    fragment = after_query[frag_pos + 1..].to_string();
                } else {
                    query = after_query.to_string();
                }
            } else if let Some(frag_pos) = full_path.find('#') {
                path = full_path[..frag_pos].to_string();
                fragment = full_path[frag_pos + 1..].to_string();
            } else {
                path = full_path.to_string();
            }
        } else {
            // No scheme, treat entire string as path
            path = url.to_string();
        }

        Some(ParsedURL {
            scheme: if scheme.is_empty() { "http".to_string() } else { scheme },
            host: if host.is_empty() { "localhost".to_string() } else { host },
            port,
            path,
            query,
            fragment,
            userinfo,
        })
    }

    /// Serialize back to URL string
    pub fn to_string(&self) -> String {
        let mut url = String::new();
        
        if !self.scheme.is_empty() {
            url.push_str(&self.scheme);
            url.push_str("://");
        }
        
        if !self.userinfo.is_empty() {
            url.push_str(&self.userinfo);
            url.push('@');
        }
        
        url.push_str(&self.host);
        
        if let Some(p) = self.port {
            url.push(':');
            url.push_str(&p.to_string());
        }
        
        if !self.path.is_empty() {
            url.push_str(&self.path);
        }
        
        if !self.query.is_empty() {
            url.push('?');
            url.push_str(&self.query);
        }
        
        if !self.fragment.is_empty() {
            url.push('#');
            url.push_str(&self.fragment);
        }
        
        url
    }

    /// Get a specific component
    pub fn get_component(&self, component: URLComponent) -> &str {
        match component {
            URLComponent::Scheme => &self.scheme,
            URLComponent::Host => &self.host,
            URLComponent::Path => &self.path,
            URLComponent::Query => &self.query,
            URLComponent::Fragment => &self.fragment,
            URLComponent::UserInfo => &self.userinfo,
            URLComponent::Port => "",
        }
    }

    /// Mutate a specific component
    pub fn mutate_component(&mut self, component: URLComponent, new_value: String) {
        match component {
            URLComponent::Scheme => self.scheme = new_value,
            URLComponent::Host => self.host = new_value,
            URLComponent::Path => self.path = new_value,
            URLComponent::Query => self.query = new_value,
            URLComponent::Fragment => self.fragment = new_value,
            URLComponent::UserInfo => self.userinfo = new_value,
            URLComponent::Port => {
                if let Ok(p) = new_value.parse::<u16>() {
                    self.port = Some(p);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_url() {
        let url = ParsedURL::parse("http://example.com/path").unwrap();
        assert_eq!(url.scheme, "http");
        assert_eq!(url.host, "example.com");
        assert_eq!(url.path, "/path");
    }

    #[test]
    fn test_parse_url_with_port() {
        let url = ParsedURL::parse("http://example.com:8080/path").unwrap();
        assert_eq!(url.port, Some(8080));
    }

    #[test]
    fn test_serialize() {
        let url = ParsedURL::parse("http://example.com/path?key=value#frag").unwrap();
        let serialized = url.to_string();
        assert!(serialized.contains("http://"));
        assert!(serialized.contains("example.com"));
        assert!(serialized.contains("/path"));
        assert!(serialized.contains("?key=value"));
        assert!(serialized.contains("#frag"));
    }
}

