/*
KL divergence measures the average extra surprise incurred 
when a model distribution Q is used instead of the true distribution P.
*/
pub fn kl_divergence_unchecked(p: &[f64], q: &[f64]) -> f64 {
    p.iter()
        .zip(q.iter())
        .filter(|(pi, _)| **pi > 0.0)
        .map(|(pi, qi)| pi * (pi / qi).ln())
        .sum()
}

/*
Total Variation Distance is the minimum amount of 
probability mass that must be moved to make two distributions identical.
*/
pub fn total_variation_distance_unchecked(p: &[f64], q: &[f64]) -> f64 {
    0.5 * p
        .iter()
        .zip(q.iter())
        .map(|(pi, qi)| (pi - qi).abs())
        .sum::<f64>()
}

// DPL measures how much the class frequency changed between two datasets.
pub fn difference_in_proportions_of_labels(train_prop: f64, test_prop: f64) -> f64 {
    (train_prop - test_prop).abs()
}