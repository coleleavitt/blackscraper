//! URL and resource validation utilities

/// Constants for file extension validation
pub const ALLOWED_EXTENSIONS: &[&str] = &[
    "html", "htm", "css", "js", "png", "jpg", "jpeg", "svg", "gif", "webp",
    "pdf", "ico", "json", "xml", "txt", "woff", "woff2", "ttf", "eot", "otf",
    "mp4", "webm", "ogg", "mp3", "wav", "asp", "php", "jsp", "shtml",
    "doc", "docx", "xls", "xlsx", "ppt", "pptx", "zip", "rar", "7z",
    "tar", "gz", "avi", "mov"
];

/// Constants for URL validation
pub const SUSPICIOUS_PATTERNS: &[&str] = &[
    "''", "'';", "%22%22", ";", 
    "autoStopperFrame.src;", "autoStopperSrc;", "'"
];


/// Validates if a resource URL is acceptable for crawling and saving
pub struct ResourceValidator;

impl ResourceValidator {
    /// Main validation entry point
    pub fn is_valid_resource_url(url: &str) -> bool {
        let path = Self::extract_path(url);
        
        Self::is_non_empty_path(&path)
            && Self::has_no_suspicious_patterns(&path)
            && Self::has_no_hidden_files(&path)
            && Self::has_valid_characters(&path)
            && Self::has_valid_extension(&path)
    }
    
    /// Extract path from URL, handling both absolute and relative URLs
    fn extract_path(url: &str) -> String {
        if let Ok(parsed) = url::Url::parse(url) {
            parsed.path().to_string()
        } else {
            url.to_string()
        }
    }
    
    /// Check if path is not empty
    fn is_non_empty_path(path: &str) -> bool {
        !path.trim().is_empty()
    }
    
    /// Check for suspicious patterns that indicate malicious or broken URLs
    fn has_no_suspicious_patterns(path: &str) -> bool {
        !SUSPICIOUS_PATTERNS.iter().any(|pattern| path.contains(pattern))
    }
    
    /// Check for hidden files (starting with .) except .well-known
    fn has_no_hidden_files(path: &str) -> bool {
        !path.split('/').any(|segment| {
            segment.starts_with('.') 
                && segment != ".well-known" 
                && !segment.is_empty()
        })
    }
    
    /// Check if path contains only allowed characters
    fn has_valid_characters(path: &str) -> bool {
        path.chars().all(|c| {
            c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '/' | '.' | '%')
        })
    }
    
    /// Validate file extension if the path represents a file
    fn has_valid_extension(path: &str) -> bool {
        if let Some(filename) = path.rsplit('/').next() {
            if filename.contains('.') && !filename.ends_with('/') {
                return Self::is_allowed_extension(filename);
            }
        }
        true // Not a file, so extension validation passes
    }
    
    /// Check if the file extension is in the allowed list
    fn is_allowed_extension(filename: &str) -> bool {
        if let Some(extension) = filename.rsplit('.').next() {
            let ext_lower = extension.to_ascii_lowercase();
            ALLOWED_EXTENSIONS.contains(&ext_lower.as_str())
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_urls() {
        assert!(ResourceValidator::is_valid_resource_url("https://example.com/page.html"));
        assert!(ResourceValidator::is_valid_resource_url("/static/style.css"));
        assert!(ResourceValidator::is_valid_resource_url("image.jpg"));
    }

    #[test]
    fn test_invalid_urls() {
        assert!(!ResourceValidator::is_valid_resource_url(""));
        assert!(!ResourceValidator::is_valid_resource_url("javascript:void(0)"));
        assert!(!ResourceValidator::is_valid_resource_url("/path/with/''"));
        assert!(!ResourceValidator::is_valid_resource_url("/.hidden/file.html"));
    }

}