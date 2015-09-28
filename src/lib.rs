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
    if a.len() == 0 && b.len() == 0 {
        return vec![];
    }

    if a.len() == 0 {
        return b.iter().map(DiffComponent::Insertion).collect();
    }

    if b.len() == 0 {
        return a.iter().map(DiffComponent::Deletion).collect();
    }

    let indexed_a: Vec<_> = a.iter()
        .enumerate()
        .map(|(i, val)| Indexed { index: i, value: val })
        .collect();
    let indexed_b: Vec<_> = b.iter()
        .enumerate()
        .map(|(i, val)| Indexed { index: i, value: val })
        .collect();

    let uniq_a = unique_elements(&indexed_a);
    let uniq_b = unique_elements(&indexed_b);

    let table = lcs::LcsTable::new(&uniq_a, &uniq_b);
    let lcs = table.longest_common_subsequence();

    if lcs.is_empty() {
        let table = lcs::LcsTable::new(&indexed_a, &indexed_b);
        return table.diff().into_iter().map(|c| {
            match c {
                lcs::DiffComponent::Insertion(elem_b) => {
                    DiffComponent::Insertion(&b[elem_b.index])
                },
                lcs::DiffComponent::Unchanged(elem_a, elem_b) => {
                    DiffComponent::Unchanged(&a[elem_a.index], &b[elem_b.index])
                },
                lcs::DiffComponent::Deletion(elem_a) => {
                    DiffComponent::Deletion(&a[elem_a.index])
                }
            }
        }).collect();
    }

    let mut ret = Vec::new();
    let mut last_index_a = 0;
    let mut last_index_b = 0;

    for (match_a, match_b) in lcs {
        let subset_a = &a[last_index_a..match_a.index];
        let subset_b = &b[last_index_b..match_b.index];

        ret.extend(patience_diff(subset_a, subset_b));

        ret.push(DiffComponent::Unchanged(match_a.value, match_b.value));

        last_index_a = match_a.index + 1;
        last_index_b = match_b.index + 1;
    }

    let subset_a = &a[last_index_a..a.len()];
    let subset_b = &b[last_index_b..b.len()];
    ret.extend(patience_diff(subset_a, subset_b));

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

    let a = vec![
        "int func1() {",
        "  return 1;",
        "}",
        "",
        "int func2() {",
        "  return 2;",
        "}"
    ];

    let b = vec![
        "int func1() {",
        "  return 1;",
        "}",
        "",
        "int func_new() {",
        "  return 0;",
        "}",
        "",
        "int func2() {",
        "  return 2;",
        "}"
    ];

    println!("");
    for x in &a { println!("{}", x); }
    println!("---");
    for x in &b { println!("{}", x); }
    println!("---");

    let table = lcs::LcsTable::new(&a, &b);
    for c in table.diff() {
        println!("{:?}", c);
    }

    println!("---");
    println!("patience");

    for c in patience_diff(&a, &b) {
        println!("{:?}", c);
    }

    panic!();
}

#[test]
fn test_unique_elements() {
    assert_eq!(vec![&2, &4, &5], unique_elements(&[1, 2, 3, 3, 4, 5, 1]));
}
