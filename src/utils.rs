extern crate num;
use self::num::Num;

use std::collections::HashMap;
use std::cmp::{Eq, PartialOrd};
use std::cmp::Ordering::Equal;
use std::hash::Hash;

pub fn count_values<I: Iterator<Item=T>, T: Eq + Hash, N: Num + Copy>(iter: I) -> HashMap<T, N> {
    let mut map = HashMap::new();
    for x in iter {
        let counter = map.entry(x).or_insert(N::zero());
        *counter = *counter + N::one();
    }
    map
}

pub fn sum_maps<T: Eq + Hash, N: Num + Copy>(mut a: HashMap<T, N>, b: HashMap<T, N>) -> HashMap<T, N> {
    for (k, b_c) in b.into_iter() {
        let counter = a.entry(k).or_insert(N::zero());
        *counter = *counter + b_c;
    }
    a
}

pub fn normalize_map<T: Eq + Hash>(counts: HashMap<T, f64>) -> HashMap<T, f64> {
    let s: f64 = counts.values().sum();
    counts.into_iter().map(|(k, c)| (k, c as f64 / s)).collect()
}

pub fn max_by_key_partial<F, I, T, A>(it:I, mut f: F) -> Option<T> where
    I: Iterator<Item=T>,
    F: FnMut(&T) -> A,
    A: PartialOrd
    {
        it.max_by(|a, b| f(a).partial_cmp(&f(b)).unwrap_or(Equal))
    }

pub fn sum_tuples<I: Iterator<Item=(A, B)>, A: Num, B:Num>(it: I) -> (A, B) {
    it.fold((A::zero(), B::zero()), |(acc_a, acc_b), (a, b)| (acc_a + a, acc_b + b))
}
