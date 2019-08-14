
// different sentence positions indicate different probability of being an important sentence
fn sentence_position(i:f64, size:usize) -> f64 {
    let normalized = i / (size as f64);
    match normalized {
        x if x.in_range(0.0, 0.1) => 0.17,
        x if x.in_range(0.1, 0.2) => 0.23,
        x if x.in_range(0.2, 0.3) => 0.14,
        x if x.in_range(0.3, 0.4) => 0.08,
        x if x.in_range(0.4, 0.5) => 0.05,
        x if x.in_range(0.5, 0.6) => 0.04,
        x if x.in_range(0.6, 0.7) => 0.06,
        x if x.in_range(0.7, 0.8) => 0.04,
        x if x.in_range(0.8, 0.9) => 0.04,
        x if x.in_range(0.9, 1.0) => 0.15,
        _ => 0.0
    }
}

trait InRange {
    fn in_range(&self, begin: Self, end: Self) -> bool;
}

impl InRange for f64 {
    fn in_range(&self, begin: f64, end: f64) -> bool {
        *self > begin && *self <= end
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
