pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn full_version() -> String {
    format!(
        "WebFirst Language Editor v{} (WFL Core v{})",
        VERSION,
        wfl_core::version::VERSION
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_format() {
        let version = full_version();
        assert!(version.contains("WebFirst Language Editor v"));
        assert!(version.contains("WFL Core v"));
    }
}
