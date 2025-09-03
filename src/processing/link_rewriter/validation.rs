//! URL validation for link rewriting

use url::Url;

/// Validates URLs to determine if they should be rewritten
pub struct RewriteValidator;

impl RewriteValidator {
    /// Check if a URL should be rewritten (skip external, javascript:, etc.)
    pub fn should_rewrite_url(url_value: &str, base_url: &Url) -> bool {
        // Skip empty URLs
        if url_value.trim().is_empty() {
            return false;
        }

        // Skip fragments
        if url_value.starts_with('#') {
            return false;
        }

        // Skip non-HTTP schemes
        if Self::is_non_http_scheme(url_value) {
            return false;
        }

        // For absolute HTTP URLs, check if they're from the same host
        if Self::is_absolute_http_url(url_value) {
            return Self::is_same_host(url_value, base_url);
        }

        // Relative URLs should be rewritten
        true
    }

    /// Check if URL uses a non-HTTP scheme that shouldn't be rewritten
    fn is_non_http_scheme(url_value: &str) -> bool {
        url_value.starts_with("javascript:") ||
        url_value.starts_with("mailto:") ||
        url_value.starts_with("data:") ||
        url_value.starts_with("tel:") ||
        url_value.starts_with("ftp:")
    }

    /// Check if URL is an absolute HTTP/HTTPS URL
    fn is_absolute_http_url(url_value: &str) -> bool {
        url_value.starts_with("http://") || url_value.starts_with("https://")
    }

    /// Check if an absolute URL is from the same host as the base URL
    fn is_same_host(url_value: &str, base_url: &Url) -> bool {
        if let Ok(parsed_url) = Url::parse(url_value) {
            parsed_url.host_str() == base_url.host_str()
        } else {
            false
        }
    }
}