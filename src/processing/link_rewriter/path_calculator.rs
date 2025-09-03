//! Relative path calculation utilities

use std::path::Path;

/// Handles relative path calculations between files
pub struct PathCalculator;

impl PathCalculator {
    /// Calculate the relative path from one file to another
    pub fn calculate_relative_path(from_path: &Path, to_path: &Path) -> String {
        log::debug!("Calculating relative path:");
        log::debug!("  From: {}", from_path.display());
        log::debug!("  To: {}", to_path.display());

        // Get parent directory of current file
        let from_dir = match from_path.parent() {
            Some(dir) => dir,
            None => {
                let result = to_path.to_string_lossy().to_string();
                log::debug!("  Result: {} (no parent directory)", result);
                return result;
            }
        };

        // Convert paths to components for comparison
        let from_components: Vec<_> = from_dir.components().collect();
        let to_components: Vec<_> = to_path.components().collect();

        // Find common prefix length
        let common_len = Self::find_common_prefix_length(&from_components, &to_components);

        // Build relative path
        let relative_parts = Self::build_relative_parts(&from_components, &to_components, common_len);

        let result = if relative_parts.is_empty() {
            // Same directory - just use filename
            Self::extract_filename(to_path)
        } else {
            relative_parts.join("/")
        };

        log::debug!("  Result: {}", result);
        result
    }

    /// Find the length of the common prefix between two component lists
    fn find_common_prefix_length(
        from_components: &[std::path::Component],
        to_components: &[std::path::Component],
    ) -> usize {
        let mut common_len = 0;
        for (from_comp, to_comp) in from_components.iter().zip(to_components.iter()) {
            if from_comp == to_comp {
                common_len += 1;
            } else {
                break;
            }
        }
        common_len
    }

    /// Build the relative path parts given the component lists and common prefix length
    fn build_relative_parts(
        from_components: &[std::path::Component],
        to_components: &[std::path::Component],
        common_len: usize,
    ) -> Vec<String> {
        let mut relative_parts = Vec::new();

        // Add ".." for each directory we need to go up from the common ancestor
        let up_levels = from_components.len() - common_len;
        for _ in 0..up_levels {
            relative_parts.push("..".to_string());
        }

        // Add path components to reach target from common ancestor
        for component in &to_components[common_len..] {
            if let Some(os_str) = component.as_os_str().to_str() {
                relative_parts.push(os_str.to_string());
            }
        }

        relative_parts
    }

    /// Extract filename from a path, with fallback
    fn extract_filename(path: &Path) -> String {
        match path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => "index.html".to_string(),
        }
    }
}