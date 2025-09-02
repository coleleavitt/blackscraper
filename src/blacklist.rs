use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Blacklist {
    pub domains: Vec<String>,
    pub urls: Vec<String>,
    pub patterns: Vec<String>,
}

impl Blacklist {
    pub fn is_blacklisted(&self, url: &str) -> bool {
        // Check exact URLs
        if self.urls.iter().any(|u| u == url) {
            return true;
        }
        // Check domain
        if let Ok(parsed) = url::Url::parse(url) {
            if let Some(host) = parsed.host_str() {
                if self.domains.iter().any(|d| host.ends_with(d)) {
                    return true;
                }
            }
        }
        // Check patterns
        for pat in &self.patterns {
            if Regex::new(pat).map(|re| re.is_match(url)).unwrap_or(false) {
                return true;
            }
        }
        false
    }
}
