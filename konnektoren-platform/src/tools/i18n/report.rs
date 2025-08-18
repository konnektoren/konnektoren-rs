#[derive(Debug)]
pub struct LanguageStats {
    pub total_keys: usize,
    pub missing_keys: usize,
    pub coverage_percentage: f64,
}
