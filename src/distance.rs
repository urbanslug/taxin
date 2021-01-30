use num::traits::Float;
use std::cmp;
use std::iter::Sum;

use indicatif::{ProgressBar, ProgressStyle};
use num_format::{Locale, ToFormattedString};

fn pairwise_distance<'a, T: 'a + Float + Sum>(
    left: impl IntoIterator<Item = &'a T>,
    right: impl IntoIterator<Item = &'a T>,
) -> T {
    left.into_iter()
        .zip(right)
        .map(|(a, b)| num::pow(*a - *b, 2))
        .sum::<T>()
        .sqrt()
}

pub fn eucledian<T: Float + Sum>(coverage_matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let samples = coverage_matrix.len();
    let columns = coverage_matrix.get(0).unwrap_or(&Vec::new()).len();

    println!(
        "Dimensions: {} samples by approx {} nodes",
        samples.to_formatted_string(&Locale::en),
        columns.to_formatted_string(&Locale::en)
    );

    // create a samples x samples distance matrix
    let mut distance_matrix: Vec<Vec<T>> = vec![vec![num::zero(); samples]; samples];

    let bar = ProgressBar::new(samples as u64);
    let t =
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})";
    bar.set_style(
        ProgressStyle::default_bar()
            .template(t)
            .progress_chars("#>-"),
    );

    // for each sample calculate its distance to every other
    for i in 0..samples {
        bar.set_position(i as u64);
        for j in i..samples {
            let dist: T = pairwise_distance(&coverage_matrix[i], &coverage_matrix[j]);
            distance_matrix[i][j] = dist;
        }
    }

    bar.finish();

    distance_matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eucledian_distance() {
        let x1 = vec![102, 3, 394, 87]
            .into_iter()
            .map(|x| x as f64)
            .collect();
        let x2 = vec![67, 83, 124, 987]
            .into_iter()
            .map(|x| x as f64)
            .collect();
        let x3 = vec![900, 27, 45, 23]
            .into_iter()
            .map(|x| x as f64)
            .collect();
        let m: Vec<Vec<f64>> = vec![x1, x2, x3];

        let k = vec![
            vec![0.0, 943.6763216272834, 873.6572554497559],
            vec![0.0, 0.0, 1277.7174961625908],
            vec![0.0, 0.0, 0.0],
        ];

        assert_eq!(eucledian(&m), k);
    }

    #[test]
    fn test_pairwise_distance() {
        // have uneven sized vectors
        let f: Vec<f64> = vec![102, 3, 394, 87]
            .into_iter()
            .map(|x| x as f64)
            .collect();
        let s: Vec<f64> = vec![67, 83, 124, 987, 823]
            .into_iter()
            .map(|x| x as f64)
            .collect();

        // Different
        let precomputed_dist: f64 = pairwise_distance(&f, &s);
        let dist: f64 = 943.6763216272834;
        assert_eq!(precomputed_dist, dist);

        // Same
        let precomputed_dist: f64 = pairwise_distance(&f, &f);
        assert_eq!(precomputed_dist, 0_f64);
    }
}

// Ignore the below code

#[allow(dead_code)]
fn pairwise_distance_loop<T: Float>(x1: Vec<T>, x2: Vec<T>) -> T {
    let mut sum: T = num::zero();

    for index in 0..cmp::min(x1.len(), x2.len()) {
        let base = x1[index] - x2[index];
        let exponent = 2;

        sum = sum + num::pow(base, exponent)
    }

    sum.sqrt()
}

#[allow(dead_code)]
fn pairwise_distance_owner<T: Float>(
    left: impl IntoIterator<Item = T>,
    right: impl IntoIterator<Item = T>,
) -> T {
    left.into_iter()
        .zip(right)
        .fold(num::zero(), |sum: T, (a, b)| sum + num::pow(a - b, 2))
        .sqrt()
}
