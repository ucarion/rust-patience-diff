extern crate lcs;

use std::collections::hash_map::{HashMap, Entry};
use std::hash::{Hash, Hasher};
use std::fmt::Debug;

#[derive(Debug, Eq)]
struct Indexed<T> {
    index: usize,
    value: T
}

impl<T> PartialEq for Indexed<T> where T: PartialEq {
    fn eq(&self, other: &Indexed<T>) -> bool {
        self.value == other.value
    }
}

impl<T> Hash for Indexed<T> where T: Hash {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.value.hash(state);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiffComponent<T> {
    Insertion(T),
    Unchanged(T, T),
    Deletion(T)
}

pub fn patience_diff<'a, T>(a: &'a [T], b: &'a [T]) -> Vec<DiffComponent<&'a T>>
        where T: Eq + Hash + Debug {
    // convert to indexed-lists
    // run core algo on total list
    // convert result into its proper type if necessary
    let a: Vec<_> = a.into_iter()
        .enumerate()
        .map(|(i, val)| Indexed { index: i, value: val })
        .collect();
    let b: Vec<_> = b.into_iter()
        .enumerate()
        .map(|(i, val)| Indexed { index: i, value: val })
        .collect();

    diff(&a, &b)
}

fn diff<'a, 'b, T>(a: &'b [Indexed<&'a T>], b: &'b [Indexed<&'a T>]) -> Vec<DiffComponent<&'a T>>
        where T: Eq + Hash + Debug {
    if a.len() == 0 && b.len() == 0 {
        return vec![];
    }

    if a.len() == 0 {
        return b.iter().map(|elem| DiffComponent::Insertion(elem.value)).collect();
    }

    if b.len() == 0 {
        return a.iter().map(|elem| DiffComponent::Deletion(elem.value)).collect();
    }

    let uniq_a = unique_elements(&a);
    let uniq_b = unique_elements(&b);

    let table = lcs::LcsTable::new(&uniq_a, &uniq_b);
    let lcs = table.longest_common_subsequence();

    if lcs.is_empty() {
        let b: Vec<_> = table.diff().iter().map(|c| {
            match *c {
                lcs::DiffComponent::Insertion(i) => DiffComponent::Insertion(i.value),
                lcs::DiffComponent::Unchanged(a, b) => DiffComponent::Unchanged(a.value, b.value),
                lcs::DiffComponent::Deletion(d) => DiffComponent::Deletion(d.value)
            }
        }).collect();

        return b;
    }

    let mut ret = Vec::new();
    let mut last_index_a = 0;
    let mut last_index_b = 0;

    for (match_a, match_b) in lcs {
        let subset_a = &a[last_index_a..match_a.index];
        let subset_b = &b[last_index_b..match_b.index];
        ret.extend(diff(subset_a, subset_b));

        ret.push(DiffComponent::Unchanged(match_a.value, match_b.value));

        last_index_a = match_a.index + 1;
        last_index_b = match_b.index + 1;
    }

    let subset_a = &a[last_index_a..a.len()];
    let subset_b = &b[last_index_b..b.len()];
    ret.extend(diff(subset_a, subset_b));

    ret
}

fn unique_elements<'a, T: Eq + Hash>(elems: &'a [T]) -> Vec<&'a T> {
    let mut counts: HashMap<&T, usize> = HashMap::new();

    for elem in elems {
        match counts.entry(elem) {
            Entry::Occupied(mut e) => {
                *e.get_mut() = e.get() + 1;
            },
            Entry::Vacant(e) => {
                e.insert(1);
            }
        }
    }

    elems.iter()
        .filter(|elem| counts.get(elem) == Some(&1))
        .collect()
}

#[test]
fn test_patience_diff() {
    let a: Vec<_> = "aax".chars().collect();
    let b: Vec<_> = "xaa".chars().collect();

    let diff = patience_diff(&a, &b);
    assert_eq!(diff, vec![
        DiffComponent::Deletion(&'a'),
        DiffComponent::Deletion(&'a'),
        DiffComponent::Unchanged(&'x', &'x'),
        DiffComponent::Insertion(&'a'),
        DiffComponent::Insertion(&'a'),
    ]);
}

#[test]
fn test_unique_elements() {
    assert_eq!(vec![&2, &4, &5], unique_elements(&[1, 2, 3, 3, 4, 5, 1]));
}
