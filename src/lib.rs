use std::collections::hash_map::{HashMap, Entry};
use std::hash::Hash;
use std::fmt::Debug;

#[derive(Debug, Eq, Hash)]
struct Indexed<T>(usize, T);

impl<T> PartialEq for Indexed<T> where T: Eq {
    fn eq(&self, other: &Indexed<T>) -> bool {
        self.1 == other.1
    }
}

enum DiffComponent<T> {
    Insertion(T),
    Unchanged(T, T),
    Deletion(T)
}

fn patience_diff<'a, T>(a: &'a [T], b: &'a [T]) -> Vec<DiffComponent<&'a T>>
        where T: Eq + Hash {
    let a: Vec<_> = a.iter().enumerate().map(|(i, val)| Indexed(i, val)).collect();
    let b: Vec<_> = b.iter().enumerate().map(|(i, val)| Indexed(i, val)).collect();

    let common_unique_elems = common_unique_elements(&a, &b);

    vec![]
}

fn common_unique_elements<'a, T: Eq + Hash>(a: &'a [T], b: &'a [T]) -> Vec<(&'a T, &'a T)> {
    let uniq_a = unique_elements(a);
    let uniq_b = unique_elements(b);

    let mut out = Vec::new();

    for elem_a in &uniq_a {
        for elem_b in &uniq_b {
            if elem_a == elem_b {
                out.push((*elem_a, *elem_b));
            }
        }
    }

    out
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
fn test_common_unique_elements() {
    let a = &[1, 2, 3];
    let b = &[1, 2, 3, 3, 4];

    let expected = vec![(&a[0], &b[0]), (&a[1], &b[1])];
    let actual = common_unique_elements(a, b);
    assert_eq!(expected, actual);
}

#[test]
fn test_unique_elements() {
    assert_eq!(vec![&2, &4, &5], unique_elements(&[1, 2, 3, 3, 4, 5, 1]));
}
