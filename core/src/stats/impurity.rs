use std::collections::{HashMap};
use std::hash::Hash;

/// Entropy:
/// measures how uncertain / unpredictable the labels are.
///
/// Interpretation:
/// → Expected surprise of a label (surprise = -log2(p))
/// → Higher = more randomness, lower = more predictable
///
/// Examples:
/// 100/0 → 0 (no uncertainty)
/// 90/10 → ~0.47 (low uncertainty)
/// 50/50 → 1.0 (max uncertainty)

pub fn entropy<T: Eq + Hash + Copy>(xs: &[T]) -> f64 {
    let n = xs.len() as f64;
    if n == 0.0 {
        return 0.0;
    }

    let freq = xs.iter().copied().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|frq| *frq+=1).or_insert(1);
        map
    });

    let h: f64 = freq.values()
        .map(|&count| count as f64 / n)
        .filter(|&p| p > 0.0)
        .map(|p| -p * p.log2())
        .sum();
    h

}

/// Gini impurity:
/// = probability of misclassification under random guessing
/// = 1 - sum(p_i^2)
/// = probability two random samples have different labels

// 100/0 (pure)
// Always guess the only class → never wrong → Gini = 0

// 90/10 (mostly pure)
// Usually right, but occasional mistakes both ways → Gini = 0.18

// 50/50 (maximally mixed)
// Frequent mistakes due to full uncertainty → Gini = 0.5

pub fn gini<T: Eq + Hash + Copy>(xs: &[T]) -> f64 {
    let n = xs.len() as f64;
    if n == 0.0 {
        return 0.0;
    }

    let freq = xs.iter().copied().fold(HashMap::new(), |mut map, v| {
        map.entry(v).and_modify(|c| *c += 1).or_insert(1);
        map
    });

    1.0 - freq.values()
        .map(|&c| {
            let p = c as f64 / n;
            p * p
        })
        .sum::<f64>()
}