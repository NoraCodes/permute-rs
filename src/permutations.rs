// SPDX-FileCopyrightText: 2020 Leonora Tindall <nora@nora.codes>
//
// SPDX-License-Identifier: AGPL-3.0-or-later WITH LicenseRef-AppStore

//! Tools for generating permutations using Heap's algorithm

fn heaps_inner<T: Clone>(k: usize, values: &mut [T]) -> Vec<Vec<T>> {
    let mut v = Vec::new();
    if k <= 1 {
        v.push(values.to_vec());
    } else {
        v.append(&mut heaps_inner(k - 1, values));
        for i in 0..(k - 1) {
            if k % 2 == 0 {
                values.swap(i, k - 1);
            } else {
                values.swap(0, k - 1);
            }
            v.append(&mut heaps_inner(k - 1, values));
        }
    }

    v
}

/// Generate permutations of a given `Vec` by copying.
///
/// # Complexity
/// This function is `O(n!)` in both memory and time.
///
/// # Determinism
/// The order of the permutations is deterministic and can be found ahead of time by
/// consulting the OEIS sequence for reverse colexicographic ordering, using
/// the appropriate elements of [A280318](https://oeis.org/A280318) as indices into
/// [A055089](https://oeis.org/A055089).
///
/// # Examples
/// ```
/// # use permute::permute;
/// assert_eq!(
///     permute(vec![1, 2, 3]),
///     vec![
///         vec![1, 2, 3],
///         vec![2, 1, 3],
///         vec![3, 1, 2],
///         vec![1, 3, 2],
///         vec![2, 3, 1],
///         vec![3, 2, 1]
///     ]
/// );
/// ```
pub fn permute<T: Clone>(values: Vec<T>) -> Vec<Vec<T>> {
    let mut values = values;
    heaps_inner(values.len(), &mut values)
}

#[test]
fn permute_numbers() {
    assert_eq!(
        permute(vec![1, 2, 3]),
        vec![
            vec![1, 2, 3],
            vec![2, 1, 3],
            vec![3, 1, 2],
            vec![1, 3, 2],
            vec![2, 3, 1],
            vec![3, 2, 1]
        ]
    );
}
