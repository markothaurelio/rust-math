use crate::stats::median_unchecked;

pub fn range_unchecked(xs: &[f64]) -> f64 {
    let min = xs.iter().copied().fold(f64::INFINITY,  |acc, x| f64::min(acc, x));
    let max = xs.iter().copied().fold(f64::NEG_INFINITY,  |acc, x| f64::max(acc, x));

    max - min
}

pub fn iqr_unchecked(xs: &[f64]) -> f64 {
    let mut v = xs.to_vec();
    v.sort_by(f64::total_cmp);

    let mid = v.len() / 2;
    println!("mid  = {}", mid);

    let (lower, upper) = if v.len() % 2 == 0 {
        (&v[..mid], &v[mid..])
    } else {
        (&v[..mid], &v[mid + 1..]) // skip the median
    };

    let q1 = median_unchecked(&lower);
    let q3 = median_unchecked(&upper);

    q3-q1

}