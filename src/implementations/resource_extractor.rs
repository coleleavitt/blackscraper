//! Resource extraction from HTML documents

use crate::implementations::url_parser::StandardUrlParser;
use crate::traits::url_parser::UrlParser;
use regex::Regex;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::sync::Arc;
use crate::blacklist::Blacklist;

/// Resource types that can be downloaded
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceType {
    Html,
    Image,
    Css,
    JavaScript,
    Font,
    Video,
    Audio,
    Document,
    Other(String),
}

/// A downloadable resource with metadata
#[derive(Debug, Clone)]
pub struct Resource {
    pub url: String,
    pub depth: usize,
}

/// Configuration for resource extraction
#[derive(Debug, Clone)]
pub struct ExtractionConfig {
    pub base_url: String,
    pub depth: usize,
    pub base_domain: String,
    pub base_path: String,
}

/// HTML element selectors grouped by resource type
#[derive(Clone)]
struct HtmlSelectors {
    html_selectors: Vec<(&'static str, &'static str)>,
    image_selectors: Vec<(&'static str, &'static str)>,
    css_selectors: Vec<(&'static str, &'static str)>,
    js_selectors: Vec<(&'static str, &'static str)>,
    font_selectors: Vec<(&'static str, &'static str)>,
    media_selectors: Vec<(&'static str, &'static str)>,
    other_selectors: Vec<(&'static str, &'static str, ResourceType)>,
}

impl HtmlSelectors {
    fn new() -> Self {
        Self {
            html_selectors: vec![
                ("a[href]", "href"),
                ("frame[src]", "src"),
                ("iframe[src]", "src"),
                ("area[href]", "href"),
            ],
            image_selectors: vec![
                ("img[src]", "src"),
                ("img[data-src]", "data-src"),
                ("picture source[srcset]", "srcset"),
                ("img[srcset]", "srcset"),
                ("link[rel='icon']", "href"),
                ("link[rel='shortcut icon']", "href"),
                ("link[rel='apple-touch-icon']", "href"),
            ],
            css_selectors: vec![
                ("link[rel='stylesheet']", "href"),
                ("link[rel=stylesheet]", "href"),
            ],
            js_selectors: vec![
                ("script[src]", "src"),
            ],
            font_selectors: vec![
                ("link[rel='preload'][as='font']", "href"),
            ],
            media_selectors: vec![
                ("audio[src]", "src"),
                ("video[src]", "src"),
                ("source[src]", "src"),
            ],
            other_selectors: vec![
                ("embed[src]", "src", ResourceType::Other("embed".to_string())),
                ("object[data]", "data", ResourceType::Other("object".to_string())),
            ],
        }
    }
}

/// Handles CSS resource extraction
#[derive(Clone)]
struct CssExtractor {
    import_regex: Regex,
    url_regex: Regex,
}

impl CssExtractor {
    fn new() -> Result<Self, regex::Error> {
        Ok(Self {
            import_regex: Regex::new(
                r#"@import\s+(?:url\s*\(\s*["']?([^"')]+)["']?\s*\)|["']([^"']+)["'])"#
            )?,
            url_regex: Regex::new(r#"url\s*\(\s*["']?([^"')]+)["']?\s*\)"#)?,
        })
    }

    fn extract_from_style_elements(
        &self,
        doc: &Html,
        processor: &mut ResourceProcessor,
        config: &ExtractionConfig,
    ) {
        if let Ok(selector) = Selector::parse("style") {
            for element in doc.select(&selector) {
                let css_content = element.text().collect::<String>();
                self.extract_from_css_content(&css_content, processor, config);
            }
        }
    }

    fn extract_from_inline_styles(
        &self,
        doc: &Html,
        processor: &mut ResourceProcessor,
        config: &ExtractionConfig,
    ) {
        if let Ok(selector) = Selector::parse("[style]") {
            for element in doc.select(&selector) {
                if let Some(style_content) = element.value().attr("style") {
                    self.extract_urls_from_css(style_content, processor, config, &ResourceType::Image);
                }
            }
        }
    }

    fn extract_from_css_content(
        &self,
        css_content: &str,
        processor: &mut ResourceProcessor,
        config: &ExtractionConfig,
    ) {
        // Extract @import rules
        for captures in self.import_regex.captures_iter(css_content) {
            if let Some(url) = captures.get(1).or_else(|| captures.get(2)) {
                let url_str = url.as_str();
                if !url_str.is_empty() {
                    processor.add_resource(url_str, &ResourceType::Css, config);
                }
            }
        }

        // Extract url() functions
        self.extract_urls_from_css(css_content, processor, config, &ResourceType::Image);
    }

    fn extract_urls_from_css(
        &self,
        css_content: &str,
        processor: &mut ResourceProcessor,
        config: &ExtractionConfig,
        default_type: &ResourceType,
    ) {
        for captures in self.url_regex.captures_iter(css_content) {
            if let Some(url_match) = captures.get(1) {
                let url_str = url_match.as_str();
                if !url_str.is_empty() {
                    let resource_type = self.determine_css_resource_type(url_str, default_type);
                    processor.add_resource(url_str, &resource_type, config);
                }
            }
        }
    }

    fn determine_css_resource_type(&self, url: &str, default_type: &ResourceType) -> ResourceType {
        if url.ends_with(".ttf")
            || url.ends_with(".woff")
            || url.ends_with(".woff2")
            || url.ends_with(".eot")
        {
            ResourceType::Font
        } else {
            default_type.clone()
        }
    }
}

/// Handles legacy HTML parsing with regex
#[derive(Clone)]
struct LegacyExtractor {
    patterns: Vec<(Regex, ResourceType)>,
    frameset_patterns: Vec<Regex>,
}

impl LegacyExtractor {
    fn new() -> Result<Self, regex::Error> {
        let pattern_specs = [
            (r#"(?i)href\s*=\s*(?:"([^"]+)"|'([^']+)'|([^\s>]+))"#, ResourceType::Html),
            (r#"(?i)src\s*=\s*(?:"([^"]+)"|'([^']+)'|([^\s>]+))"#, ResourceType::Other("unknown".to_string())),
            (r#"(?i)data\s*=\s*(?:"([^"]+)"|'([^']+)'|([^\s>]+))"#, ResourceType::Other("object".to_string())),
        ];

        let patterns = pattern_specs
            .iter()
            .map(|(pattern, resource_type)| {
                Regex::new(pattern).map(|regex| (regex, resource_type.clone()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let frameset_patterns = [
            r#"(?i)<frame\s+[^>]*src\s*=\s*["']?([^"'\s>]+)["']?[^>]*>"#,
            r#"(?i)<iframe\s+[^>]*src\s*=\s*["']?([^"'\s>]+)["']?[^>]*>"#,
        ]
            .iter()
            .map(|pattern| Regex::new(pattern))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            patterns,
            frameset_patterns,
        })
    }

    fn extract_resources(
        &self,
        html: &str,
        processor: &mut ResourceProcessor,
        config: &ExtractionConfig,
    ) {
        self.extract_with_patterns(html, processor, config);
        self.extract_framesets(html, processor, config);
    }

    fn extract_with_patterns(
        &self,
        html: &str,
        processor: &mut ResourceProcessor,
        config: &ExtractionConfig,
    ) {
        for (regex, default_type) in &self.patterns {
            for captures in regex.captures_iter(html) {
                if let Some(url) = self.extract_first_match(&captures) {
                    if self.is_valid_url(&url, &processor.url_parser) {
                        let resource_type = ResourceTypeGuesser::guess_from_url(&url, default_type);
                        processor.add_resource(&url, &resource_type, config);
                    }
                }
            }
        }
    }

    fn extract_framesets(
        &self,
        html: &str,
        processor: &mut ResourceProcessor,
        config: &ExtractionConfig,
    ) {
        for regex in &self.frameset_patterns {
            for captures in regex.captures_iter(html) {
                if let Some(url_match) = captures.get(1) {
                    let url = url_match.as_str().trim();
                    if self.is_valid_url(url, &processor.url_parser) {
                        processor.add_resource(url, &ResourceType::Html, config);
                    }
                }
            }
        }
    }

    fn extract_first_match(&self, captures: &regex::Captures) -> Option<String> {
        captures
            .iter()
            .skip(1)
            .find_map(|m| m)
            .map(|m| m.as_str().trim().to_string())
            .filter(|s| !s.is_empty())
    }

    fn is_valid_url(&self, url: &str, url_parser: &StandardUrlParser) -> bool {
        !url.is_empty()
            && !url_parser.is_event_handler(url)
            && !url_parser.is_invalid_url_pattern(url)
    }
}

/// Handles resource type detection from URLs and MIME types
struct ResourceTypeGuesser;

impl ResourceTypeGuesser {
    fn guess_from_url(url: &str, default_type: &ResourceType) -> ResourceType {
        let lower_url = url.to_lowercase();

        if Self::is_html_file(&lower_url) {
            ResourceType::Html
        } else if Self::is_css_file(&lower_url) {
            ResourceType::Css
        } else if Self::is_js_file(&lower_url) {
            ResourceType::JavaScript
        } else if Self::is_image_file(&lower_url) {
            ResourceType::Image
        } else if Self::is_font_file(&lower_url) {
            ResourceType::Font
        } else if Self::is_video_file(&lower_url) {
            ResourceType::Video
        } else if Self::is_audio_file(&lower_url) {
            ResourceType::Audio
        } else if Self::is_document_file(&lower_url) {
            ResourceType::Document
        } else {
            default_type.clone()
        }
    }

    fn is_html_file(url: &str) -> bool {
        url.ends_with(".html") || url.ends_with(".htm") || url.ends_with(".asp")
            || url.ends_with(".php") || url.ends_with(".jsp") || url.ends_with(".shtml")
    }

    fn is_css_file(url: &str) -> bool {
        url.ends_with(".css")
    }

    fn is_js_file(url: &str) -> bool {
        url.ends_with(".js")
    }

    fn is_image_file(url: &str) -> bool {
        url.ends_with(".jpg") || url.ends_with(".jpeg") || url.ends_with(".png")
            || url.ends_with(".gif") || url.ends_with(".svg") || url.ends_with(".webp")
    }

    fn is_font_file(url: &str) -> bool {
        url.ends_with(".ttf") || url.ends_with(".woff") || url.ends_with(".woff2")
            || url.ends_with(".eot")
    }

    fn is_video_file(url: &str) -> bool {
        url.ends_with(".mp4") || url.ends_with(".webm") || url.ends_with(".ogg")
            || url.ends_with(".avi") || url.ends_with(".mov")
    }

    fn is_audio_file(url: &str) -> bool {
        url.ends_with(".mp3") || url.ends_with(".wav")
    }

    fn is_document_file(url: &str) -> bool {
        url.ends_with(".pdf") || url.ends_with(".doc") || url.ends_with(".docx")
            || url.ends_with(".xls") || url.ends_with(".xlsx") || url.ends_with(".ppt")
            || url.ends_with(".pptx") || url.ends_with(".zip") || url.ends_with(".rar")
            || url.ends_with(".7z") || url.ends_with(".tar") || url.ends_with(".gz")
            || url.ends_with(".json") || url.ends_with(".xml")
    }
}

/// Handles URL processing and validation
pub(crate) struct ResourceProcessor {
    url_parser: StandardUrlParser,
    resources: Vec<Resource>,
    seen_urls: HashSet<String>,
    blacklist: Arc<Blacklist>,
}

impl ResourceProcessor {
    fn new(blacklist: Arc<Blacklist>) -> Self {
        Self {
            url_parser: StandardUrlParser,
            resources: Vec::new(),
            seen_urls: HashSet::new(),
            blacklist,
        }
    }

    fn is_recursive_path(url: &str) -> bool {
        // Check for repeated path segments (e.g., /HTTP/HTTP/HTTP/...)
        let path = if let Ok(parsed) = url::Url::parse(url) {
            parsed.path().to_string()
        } else {
            // For relative URLs, just use the string
            if let Some(idx) = url.find('?') {
                url[..idx].to_string()
            } else {
                url.to_string()
            }
        };
        let segments: Vec<_> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut counts = std::collections::HashMap::new();
        for seg in &segments {
            *counts.entry(seg).or_insert(0) += 1;
            // Block if any segment repeats more than 2 times (tune as needed)
            if counts[seg] > 2 {
                // eprintln!("[BLOCKED: recursive segment] {}", url);
                return true;
            }
        }
        // Block if path has more than 6 segments (tune as needed)
        if segments.len() > 6 {
            // eprintln!("[BLOCKED: too many segments] {}", url);
            return true;
        }
        // Block if any segment is all uppercase and longer than 3 chars (likely placeholder)
        for seg in &segments {
            if seg.len() > 3 && seg.chars().all(|c| c.is_ascii_uppercase()) {
                // eprintln!("[BLOCKED: suspicious segment] {}", url);
                return true;
            }
        }
        false
    }

    fn is_invalid_url(&self, url: &str) -> bool {
        if self.blacklist.is_blacklisted(url) {
            return true;
        }
        self.url_parser.is_event_handler(url)
            || self.url_parser.is_invalid_url_pattern(url)
            || self.url_parser.is_recursive_url(url)
            || Self::is_recursive_path(url)
            || !Self::is_valid_resource_url(url)
    }

    /// Helper to check if a resource URL is valid (matches SiteSaver logic)
    fn is_valid_resource_url(url: &str) -> bool {
        // Parse the URL to get the path
        let path_owned;
        let path: &str = if let Ok(parsed) = url::Url::parse(url) {
            path_owned = parsed.path().to_string();
            &path_owned
        } else {
            url
        };
        // Reject empty, only punctuation, or suspicious paths
        if path.trim().is_empty() {
            return false;
        }
        // Reject paths that are just quotes, semicolons, or similar
        let suspicious = ["''", "'';", "%22%22", ";", "autoStopperFrame.src;", "autoStopperSrc;", "'"].iter();
        if suspicious.clone().any(|s| path.contains(s)) {
            return false;
        }
        // Split the path into segments and check for dotfiles (except .well-known)
        for segment in path.split('/') {
            if segment.starts_with('.') && segment != ".well-known" && !segment.is_empty() {
                return false;
            }
        }
        // Only allow paths with alphanumeric, dash, underscore, slash, and dot
        let valid = path.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '/' || c == '.' );
        if !valid {
            return false;
        }
        // If the path looks like a file (contains a dot and doesn't end with a slash), check extension
        if let Some(last) = path.rsplit('/').next() {
            if last.contains('.') && !last.ends_with('/') {
                let allowed_exts = [
                    "html", "htm", "css", "js", "png", "jpg", "jpeg", "svg", "gif", "webp", "pdf", "ico", "json", "xml", "txt", "woff", "woff2", "ttf", "eot", "otf", "mp4", "webm", "ogg", "mp3", "wav"
                ];
                if let Some(ext) = last.rsplit('.').next() {
                    if !allowed_exts.contains(&ext.to_ascii_lowercase().as_str()) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        true
    }

    fn add_resource(&mut self, url: &str, resource_type: &ResourceType, config: &ExtractionConfig) {
        if self.is_invalid_url(url) {
            return;
        }

        if let Some(resolved_url) = self.url_parser.resolve_url(&config.base_url, url) {
            let normalized_url = self.url_parser.normalize_url(&resolved_url);

            if self.seen_urls.contains(&normalized_url) {
                return;
            }

            self.seen_urls.insert(normalized_url.clone());

            let fixed_url = self.fix_url_issues(&normalized_url);

            if self.should_include_resource(resource_type, &fixed_url, config) {
                self.resources.push(Resource {
                    url: fixed_url,
                    depth: config.depth,
                });
            }
        }
    }

    fn process_srcset(&mut self, srcset: &str, resource_type: &ResourceType, config: &ExtractionConfig) {
        for src_part in srcset.split(',') {
            if let Some(url) = src_part.trim().split_whitespace().next() {
                if !url.is_empty() {
                    self.add_resource(url, resource_type, config);
                }
            }
        }
    }


    fn should_include_resource(&self, resource_type: &ResourceType, url: &str, config: &ExtractionConfig) -> bool {
        match resource_type {
            ResourceType::Html => {
                self.url_parser.should_crawl(url, &config.base_domain, &config.base_path)
            }
            _ => true, // Always include non-HTML resources
        }
    }

    fn fix_url_issues(&self, url: &str) -> String {
        use url::Url;

        let mut fixed_url = url.trim().to_string();

        if let Ok(mut parsed_url) = Url::parse(&fixed_url) {
            let path = parsed_url.path();
            if path.contains("//") {
                let new_path = path.replace("//", "/");
                parsed_url.set_path(&new_path);
                fixed_url = parsed_url.to_string();
            }
        }

        fixed_url
    }

    fn into_resources(self) -> Vec<Resource> {
        self.resources
    }
}

/// Handles JS resource extraction
#[derive(Clone)]
pub struct JsExtractor {
    url_regex: Regex,
}

impl JsExtractor {
    pub fn new() -> Result<Self, regex::Error> {
        Ok(Self {
            // Matches URLs in JS: '...', "...", fetch('...'), src: '...', etc.
            url_regex: Regex::new(r#"["'](https?://[^\s"']+|/[^\s"']+|\.[^\s"']+|[a-zA-Z0-9_\-/]+\.[a-zA-Z0-9]+)["']"#)?,
        })
    }

    pub fn extract_from_js_content(
        &self,
        js_content: &str,
        processor: &mut ResourceProcessor,
        config: &ExtractionConfig,
    ) {
        for cap in self.url_regex.captures_iter(js_content) {
            if let Some(url) = cap.get(1) {
                let url_str = url.as_str();
                if !url_str.is_empty() {
                    processor.add_resource(url_str, &ResourceType::Other("js-embedded".to_string()), config);
                }
            }
        }
    }
}

/// Main resource extractor
#[derive(Clone)]
pub struct ResourceExtractor {
    selectors: HtmlSelectors,
    css_extractor: CssExtractor,
    legacy_extractor: LegacyExtractor,
    blacklist: Arc<Blacklist>,
}

impl ResourceExtractor {
    pub fn new(blacklist: Arc<Blacklist>) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            selectors: HtmlSelectors::new(),
            css_extractor: CssExtractor::new()?,
            legacy_extractor: LegacyExtractor::new()?,
            blacklist,
        })
    }

    /// Extract all resources from an HTML document
    pub fn extract_resources(
        &self,
        doc: &Html,
        base: &str,
        next_depth: usize,
        base_domain: &str,
        base_path: &str,
    ) -> Vec<Resource> {
        let config = ExtractionConfig {
            base_url: base.to_string(),
            depth: next_depth,
            base_domain: base_domain.to_string(),
            base_path: base_path.to_string(),
        };

        let mut processor = ResourceProcessor::new(self.blacklist.clone());
        let js_extractor = JsExtractor::new().expect("Failed to create JsExtractor");

        self.extract_html_links(doc, &mut processor, &config);
        self.extract_image_links(doc, &mut processor, &config);
        self.extract_css_links(doc, &mut processor, &config);
        self.extract_js_links(doc, &mut processor, &config);
        self.extract_font_links(doc, &mut processor, &config);
        self.extract_media_links(doc, &mut processor, &config);
        self.extract_other_links(doc, &mut processor, &config);
        self.css_extractor.extract_from_style_elements(doc, &mut processor, &config);
        self.css_extractor.extract_from_inline_styles(doc, &mut processor, &config);

        // Extract URLs from inline <script> tags
        if let Ok(selector) = Selector::parse("script:not([src])") {
            for element in doc.select(&selector) {
                let js_content = element.text().collect::<String>();
                js_extractor.extract_from_js_content(&js_content, &mut processor, &config);
            }
        }

        // Extract URLs from event/data attributes
        Self::extract_event_and_data_attrs(doc, &mut processor, &config);

        processor.into_resources()
    }

    /// Extract URLs from event handler and data-* attributes
    fn extract_event_and_data_attrs(doc: &Html, processor: &mut ResourceProcessor, config: &ExtractionConfig) {
        let url_regex = Regex::new(r#"https?://[^\s'"<>]+|/[^\s'"<>]+"#).unwrap();
        for element in doc.tree.nodes() {
            if let scraper::Node::Element(node) = element.value() {
                for (attr, value) in node.attrs.iter() {
                    let attr_lc = attr.local.as_ref().to_ascii_lowercase();
                    if attr_lc.starts_with("on") || attr_lc.starts_with("data-") {
                        for url_match in url_regex.find_iter(value) {
                            let url_str = url_match.as_str();
                            if !url_str.is_empty() {
                                processor.add_resource(url_str, &ResourceType::Other("event-or-data-attr".to_string()), config);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Extract resources from legacy HTML using regex patterns
    pub fn extract_legacy_resources(
        &self,
        html: &str,
        base: &str,
        next_depth: usize,
        base_domain: &str,
        base_path: &str,
    ) -> Vec<Resource> {
        let config = ExtractionConfig {
            base_url: base.to_string(),
            depth: next_depth,
            base_domain: base_domain.to_string(),
            base_path: base_path.to_string(),
        };

        let mut processor = ResourceProcessor::new(self.blacklist.clone());
        self.legacy_extractor.extract_resources(html, &mut processor, &config);

        processor.into_resources()
    }

    /// Extract HTML links (a, frame, iframe, area)
    fn extract_html_links(&self, doc: &Html, processor: &mut ResourceProcessor, config: &ExtractionConfig) {
        self.extract_by_type(doc, &self.selectors.html_selectors, &ResourceType::Html, processor, config);
    }

    /// Extract image links (img, picture, icon, etc.)
    fn extract_image_links(&self, doc: &Html, processor: &mut ResourceProcessor, config: &ExtractionConfig) {
        self.extract_by_type(doc, &self.selectors.image_selectors, &ResourceType::Image, processor, config);
    }

    /// Extract CSS links (link rel=stylesheet)
    fn extract_css_links(&self, doc: &Html, processor: &mut ResourceProcessor, config: &ExtractionConfig) {
        self.extract_by_type(doc, &self.selectors.css_selectors, &ResourceType::Css, processor, config);
    }

    /// Extract JavaScript links (script[src])
    fn extract_js_links(&self, doc: &Html, processor: &mut ResourceProcessor, config: &ExtractionConfig) {
        self.extract_by_type(doc, &self.selectors.js_selectors, &ResourceType::JavaScript, processor, config);
    }

    /// Extract font links (link rel=preload as=font)
    fn extract_font_links(&self, doc: &Html, processor: &mut ResourceProcessor, config: &ExtractionConfig) {
        self.extract_by_type(doc, &self.selectors.font_selectors, &ResourceType::Font, processor, config);
    }

    /// Extract media links (audio, video, source)
    fn extract_media_links(&self, doc: &Html, processor: &mut ResourceProcessor, config: &ExtractionConfig) {
        self.extract_by_type(doc, &self.selectors.media_selectors, &ResourceType::Other("media".to_string()), processor, config);
    }

    /// Extract other resource links (embed, object, etc.)
    fn extract_other_links(&self, doc: &Html, processor: &mut ResourceProcessor, config: &ExtractionConfig) {
        for (selector_str, attr, resource_type) in &self.selectors.other_selectors {
            self.extract_by_type(doc, &[(selector_str, attr)], resource_type, processor, config);
        }
    }

    /// Extract resources by selector and attribute
    fn extract_by_type(
        &self,
        doc: &Html,
        selectors: &[(&str, &str)],
        resource_type: &ResourceType,
        processor: &mut ResourceProcessor,
        config: &ExtractionConfig,
    ) {
        for (selector_str, attr) in selectors {
            let selector = match Selector::parse(selector_str) {
                Ok(sel) => sel,
                Err(e) => {
                    eprintln!("[ResourceExtractor] Failed to parse selector '{}': {}", selector_str, e);
                    continue;
                }
            };
            for element in doc.select(&selector) {
                self.process_element_attr(element, attr, resource_type, processor, config);
            }
        }
    }

    /// Process a single element's attribute for resource extraction
    fn process_element_attr(
        &self,
        element: scraper::ElementRef,
        attr: &str,
        resource_type: &ResourceType,
        processor: &mut ResourceProcessor,
        config: &ExtractionConfig,
    ) {
        if let Some(url_attr) = element.value().attr(attr) {
            if attr == "srcset" {
                processor.process_srcset(url_attr, resource_type, config);
            } else {
                processor.add_resource(url_attr, resource_type, config);
            }
        }
    }
}
