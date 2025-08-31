//! URL parser implementation with enhanced URL handling

use crate::traits::url_parser::UrlParser;
use html_escape::decode_html_entities;
use url::Url;

/// Standard URL parser implementation with enhanced URL handling
pub struct StandardUrlParser;

impl UrlParser for StandardUrlParser {
    fn resolve_url(&self, base: &str, href: &str) -> Option<String> {
        // First check if this is a JavaScript event handler or similar non-URL
        if self.is_event_handler(href) || self.is_invalid_url_pattern(href) {
            return None;
        }

        // Decode HTML entities like &amp; to &
        let decoded_href = decode_html_entities(href).into_owned();
        let decoded_href = decoded_href.trim();

        // Absolute URLs
        if decoded_href.starts_with("http://") || decoded_href.starts_with("https://") {
            return Some(self.normalize_url(&decoded_href));
        }

        // Protocol-relative URLs (//example.com/path)
        if decoded_href.starts_with("//") {
            if let Ok(base_url) = Url::parse(base) {
                let protocol = base_url.scheme();
                return Some(self.normalize_url(&format!("{}:{}", protocol, decoded_href)));
            }
            return None;
        }

        // Skip invalid URLs
        if decoded_href.starts_with('#') ||
            decoded_href.starts_with("javascript:") ||
            decoded_href.starts_with("data:") ||
            decoded_href.starts_with("mailto:") ||
            decoded_href.is_empty() {
            return None;
        }

        // Parse the base URL
        let base_url = match Url::parse(base) {
            Ok(url) => url,
            Err(_) => {
                // Try to recover by prepending protocol if missing
                if !base.contains("://") && !base.is_empty() {
                    if let Ok(fixed_url) = Url::parse(&format!("http://{}", base)) {
                        fixed_url
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
        };

        // Handle relative URLs
        match base_url.join(&decoded_href) {
            Ok(mut url) => {
                url.set_fragment(None);
                Some(self.normalize_url(&url.to_string()))
            }
            Err(_) => None,
        }
    }

    fn should_crawl(&self, url: &str, base_domain: &str, base_path: &str) -> bool {
        if let Ok(u) = Url::parse(url) {
            if let Some(host) = u.host_str() {
                // More flexible domain matching for subdomains
                if host == base_domain
                    || (host.ends_with(base_domain)
                    && host.len() > base_domain.len()
                    && host.chars().nth(host.len() - base_domain.len() - 1) == Some('.'))
                {
                    return u.path().starts_with(base_path);
                }
            }
        }
        false
    }
}

impl StandardUrlParser {
    /// Detect JavaScript event handlers that are mistakenly being treated as URLs
    pub fn is_event_handler(&self, href: &str) -> bool {
        let event_patterns = [
            "click:", "focus:", "blur:", "mousedown:", "mouseup:",
            "mouseover:", "mouseout:", "mousemove:", "mouseenter:", "mouseleave:",
            "keydown:", "keyup:", "keypress:", "touchstart:", "touchend:", "touchmove:",
            "transitionend:", "animationend:", "focusin:", "focusout:",
            "drag", "drop", "contextmenu:",
        ];

        // If it contains a colon followed by a semicolon, it's likely an event handler
        if href.contains(":") && href.contains(";") {
            return true;
        }

        // Check for specific event handler patterns
        for pattern in &event_patterns {
            if href.contains(pattern) {
                return true;
            }
        }

        // Check for complex patterns with parentheses (function calls)
        href.contains("(") && href.contains(")")
    }

    /// Detect other invalid URL patterns
    pub fn is_invalid_url_pattern(&self, href: &str) -> bool {
        href.len() > 200
            || href.chars().filter(|&c| c == ':').count() > 1
            || href.chars().filter(|&c| c == ';').count() > 3
    }

    /// Normalize a URL to prevent infinite recursion and duplicates
    pub fn normalize_url(&self, url: &str) -> String {
        if let Ok(mut parsed_url) = Url::parse(url) {
            // Remove fragments
            parsed_url.set_fragment(None);

            // Normalize URL path
            let path = parsed_url.path();
            if path.contains("//") || path.contains("./") || path.contains("/.") {
                if let Ok(new_url) = parsed_url.join("") {
                    parsed_url = new_url;
                }
            }

            // Remove trailing slash (unless it's the root path)
            let path = parsed_url.path().to_string();
            if path != "/" && path.ends_with('/') {
                parsed_url.set_path(&path[..path.len() - 1]);
            }

            // Sort query parameters to ensure consistent ordering
            if let Some(query) = parsed_url.query() {
                if !query.is_empty() {
                    let mut params: Vec<(String, String)> = Vec::new();
                    for (key, value) in parsed_url.query_pairs() {
                        params.push((key.into_owned(), value.into_owned()));
                    }

                    params.sort_by(|a, b| a.0.cmp(&b.0));

                    let sorted_query = params
                        .iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect::<Vec<String>>()
                        .join("&");

                    parsed_url.set_query(Some(&sorted_query));
                }
            }

            return parsed_url.to_string();
        }

        url.to_string()
    }

    /// Check if a URL might cause an infinite recursion
    pub fn is_recursive_url(&self, url: &str) -> bool {
        let patterns = ["amp;amp;amp;", "amp;amp;"];

        for pattern in &patterns {
            if url.contains(pattern) {
                return true;
            }
        }

        // Look for repeating path segments
        if let Ok(parsed_url) = Url::parse(url) {
            if let Some(path) = parsed_url.path_segments() {
                let segments: Vec<&str> = path.collect();

                if segments.len() > 3 {
                    for i in 2..segments.len() {
                        if segments[i] == segments[i - 1] && segments[i] == segments[i - 2] {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }
}