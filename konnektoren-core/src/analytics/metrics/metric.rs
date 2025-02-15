pub trait Metric {
    fn name(&self) -> &str;
    fn value(&self) -> f64;
    fn description(&self) -> &str;
}
