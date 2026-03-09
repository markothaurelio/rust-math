use core::f64;

use crate::stats::{mean_unchecked, sample_standard_deviation_unchecked};

/*
KL divergence measures the average extra surprise incurred 
when a model distribution Q is used instead of the true distribution P.

Useful when you want to measure how much information is lost 
when approximating the true distribution 𝑃 with a model distribution 𝑄

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

Useful when you want a simple, bounded measure of how different two
probability distributions are in terms of total probability mass that must shift (like: dataset shift detection).

*/
pub fn total_variation_distance_unchecked(p: &[f64], q: &[f64]) -> f64 {
    0.5 * p
        .iter()
        .zip(q.iter())
        .map(|(pi, qi)| (pi - qi).abs())
        .sum::<f64>()
}

// DPL measures how much the class frequency changed between two datasets.
// Useful for detecting label distribution drift between training and test datasets.
pub fn difference_in_proportions_of_labels(train_prop: f64, test_prop: f64) -> f64 {
    (train_prop - test_prop).abs()
}

pub fn skew_unchecked(xs: &[f64]) -> f64 {

    let n = xs.len() as f64;
    let mean = mean_unchecked(xs);
    let std = sample_standard_deviation_unchecked(xs);

    let bias_correction  = n / ((n - 1.0) * (n - 2.0));

    let standardized_cubic_sum = xs.iter().map(|&x| ((x - mean) / std).powi(3)).sum::<f64>();

    bias_correction * standardized_cubic_sum

}