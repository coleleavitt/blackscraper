//! Resource extraction from HTML documents

use crate::implementations::url_parser::StandardUrlParser;
use crate::traits::url_parser::UrlParser;
use regex::Regex;
use scraper::{Html, Selector};
use std::collections::HashSet;

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
    pub resource_type: ResourceType,
    pub depth: usize,
    pub referrer: String,
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
struct CssExtractor {
    import_regex: Regex,
    url_regex: Regex,
}

impl CssExtractor {
    fn new() -> Result<Self, regex::Error> {
        Ok(Self {
            import_regex: Regex::new(
                r#"@import\s+(?:url\s*\(\s*["']?([^"'\)]+)["']?\s*\)|["']([^"']+)["'])"#
            )?,
            url_regex: Regex::new(r#"url\s*\(\s*["']?([^"'\)]+)["']?\s*\)"#)?,
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

    fn from_mime_type(mime_type: &str, fallback_url: &str) -> ResourceType {
        if mime_type.starts_with("text/html") {
            ResourceType::Html
        } else if mime_type.starts_with("text/css") {
            ResourceType::Css
        } else if mime_type.starts_with("image/") {
            ResourceType::Image
        } else if mime_type.starts_with("application/javascript") || mime_type.starts_with("text/javascript") {
            ResourceType::JavaScript
        } else if mime_type.starts_with("font/") || mime_type.contains("font") {
            ResourceType::Font
        } else if mime_type.starts_with("video/") {
            ResourceType::Video
        } else if mime_type.starts_with("audio/") {
            ResourceType::Audio
        } else if mime_type.starts_with("application/pdf") {
            ResourceType::Document
        } else {
            Self::guess_from_url(fallback_url, &ResourceType::Other("unknown".to_string()))
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
            || url.ends_with(".pptx")
    }
}

/// Handles URL processing and validation
struct ResourceProcessor {
    url_parser: StandardUrlParser,
    resources: Vec<Resource>,
    seen_urls: HashSet<String>,
}

impl ResourceProcessor {
    fn new() -> Self {
        Self {
            url_parser: StandardUrlParser,
            resources: Vec::new(),
            seen_urls: HashSet::new(),
        }
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
                    resource_type: resource_type.clone(),
                    depth: config.depth,
                    referrer: config.base_url.clone(),
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

    fn is_invalid_url(&self, url: &str) -> bool {
        self.url_parser.is_event_handler(url)
            || self.url_parser.is_invalid_url_pattern(url)
            || self.url_parser.is_recursive_url(url)
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

/// Main resource extractor
pub struct ResourceExtractor {
    selectors: HtmlSelectors,
    css_extractor: CssExtractor,
    legacy_extractor: LegacyExtractor,
}

impl ResourceExtractor {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            selectors: HtmlSelectors::new(),
            css_extractor: CssExtractor::new()?,
            legacy_extractor: LegacyExtractor::new()?,
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

        let mut processor = ResourceProcessor::new();

        self.extract_html_resources(doc, &mut processor, &config);
        self.css_extractor.extract_from_style_elements(doc, &mut processor, &config);
        self.css_extractor.extract_from_inline_styles(doc, &mut processor, &config);

        processor.into_resources()
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

        let mut processor = ResourceProcessor::new();
        self.legacy_extractor.extract_resources(html, &mut processor, &config);

        processor.into_resources()
    }

    /// Determine the MIME type from a URL or content
    pub fn detect_mime_type(&self, url: &str, content_type: Option<&str>) -> ResourceType {
        if let Some(mime) = content_type {
            ResourceTypeGuesser::from_mime_type(mime, url)
        } else {
            ResourceTypeGuesser::guess_from_url(url, &ResourceType::Other("unknown".to_string()))
        }
    }

    fn extract_html_resources(
        &self,
        doc: &Html,
        processor: &mut ResourceProcessor,
        config: &ExtractionConfig,
    ) {
        self.extract_by_type(doc, &self.selectors.html_selectors, &ResourceType::Html, processor, config);
        self.extract_by_type(doc, &self.selectors.image_selectors, &ResourceType::Image, processor, config);
        self.extract_by_type(doc, &self.selectors.css_selectors, &ResourceType::Css, processor, config);
        self.extract_by_type(doc, &self.selectors.js_selectors, &ResourceType::JavaScript, processor, config);
        self.extract_by_type(doc, &self.selectors.font_selectors, &ResourceType::Font, processor, config);
        self.extract_by_type(doc, &self.selectors.media_selectors, &ResourceType::Other("media".to_string()), processor, config);

        for (selector_str, attr, resource_type) in &self.selectors.other_selectors {
            self.extract_by_type(doc, &[(selector_str, attr)], resource_type, processor, config);
        }
    }

    fn extract_by_type(
        &self,
        doc: &Html,
        selectors: &[(&str, &str)],
        resource_type: &ResourceType,
        processor: &mut ResourceProcessor,
        config: &ExtractionConfig,
    ) {
        for (selector_str, attr) in selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in doc.select(&selector) {
                    if let Some(url_attr) = element.value().attr(attr) {
                        if *attr == "srcset" {
                            processor.process_srcset(url_attr, resource_type, config);
                        } else {
                            processor.add_resource(url_attr, resource_type, config);
                        }
                    }
                }
            }
        }
    }
}

impl Default for ResourceExtractor {
    fn default() -> Self {
        Self::new().expect("Failed to create ResourceExtractor")
    }
}