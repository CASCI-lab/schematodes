use itertools::Itertools;
use pyo3::prelude::*;
use std::collections::{HashMap, HashSet};

/// A Python module implemented in Rust.
#[pymodule]
fn schematodes(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(schemer, m)?)?;
    m.add_class::<TwoSymbolSchema>()?;
    Ok(())
}

/// A Python class implemented in Rust. This is the primary return type of the
/// module. See stub file for Python usage. The class contains three fields:
/// `redescribed_schemata`, `bubble_indices`, and `signature`. These are the set
/// of one-symbol schemata that are redescribed, the indices of the bubbles, and
/// the signature (number of instances of each unique symbol) of the schema.
#[derive(Hash, Eq, PartialEq, Clone, Debug, Default)]
#[pyclass]
struct TwoSymbolSchema {
    redescribed_schemata: Vec<Vec<u8>>,
    bubble_indices: Vec<Vec<usize>>,
    signature: Vec<usize>,
}

#[pymethods]
impl TwoSymbolSchema {
    #[new]
    fn py_new(
        redescribed_schemata: Vec<Vec<u8>>,
        bubble_indices: Vec<Vec<usize>>,
        signature: Vec<usize>,
    ) -> Self {
        Self {
            redescribed_schemata,
            bubble_indices,
            signature,
        }
    }
    #[getter]
    fn get_redescribed_schemata(&self) -> Vec<Vec<u8>> {
        self.redescribed_schemata.clone()
    }
    #[getter]
    fn get_bubble_indices(&self) -> Vec<Vec<usize>> {
        self.bubble_indices.clone()
    }

    #[staticmethod]
    fn trivial(
        redescribed_schemata: Vec<Vec<u8>>,
        signature: Option<Vec<usize>>,
        max_symbol: Option<usize>,
    ) -> TwoSymbolSchema {
        if let Some(s) = signature {
            TwoSymbolSchema {
                redescribed_schemata,
                bubble_indices: vec![],
                signature: s,
            }
        } else {
            let s: Vec<usize>;
            // assume that the signature is the same for all schema
            if let Some(m) = max_symbol {
                s = schema_signature(&redescribed_schemata[0], m);
            } else {
                s = vec![];
            }
            TwoSymbolSchema {
                redescribed_schemata,
                bubble_indices: vec![],
                signature: s,
            }
        }
    }
}

/// This is the function that will be used in Python to redescribe a set of
/// one-symbol schemata as a list of two-symbol schemata.
#[pyfunction]
fn schemer(pis: Vec<Vec<u8>>, max_symbol: Option<usize>) -> Vec<TwoSymbolSchema> {
    let max_symbol = max_symbol.unwrap_or_else(|| compute_max_symbol(&pis));

    let mut tss_vec: Vec<TwoSymbolSchema> = Vec::new();

    // gather one-symbol schemata by the number of instances of each unique symbol
    // in the schema (we call this the signature).
    let mut schemata_with_signature: HashMap<Vec<usize>, Vec<Vec<u8>>> = HashMap::new();
    for pi in pis {
        let signature = schema_signature(&pi, max_symbol);
        if schemata_with_signature.contains_key(&signature) {
            let mut pi_vec: Vec<Vec<u8>> = schemata_with_signature[&signature].clone();
            pi_vec.push(pi);
            schemata_with_signature.insert(signature, pi_vec);
        } else {
            schemata_with_signature.insert(signature, vec![pi]);
        }
    }

    // Loop through the unique signatures and compress the corresponding schemata
    // for each.
    for (signature, pi) in schemata_with_signature {
        let tss = tss_for_one_symbol_schemata_with_signature(&pi, signature);
        tss_vec.extend(tss);
    }
    tss_vec
}

/// Compress a list of one symbol schemata that have the same signature. This
/// function does not verify that the signatures are equal, and will give
/// incorrect results if they are not. Returns a vector of `TwoSymbolSchema`
/// objects corresponding to a one symbol schema action of a product of
/// symmetric one symbol schemata.
fn tss_for_one_symbol_schemata_with_signature(
    one_symbol_schemata: &Vec<Vec<u8>>,
    signature: Vec<usize>,
) -> Vec<TwoSymbolSchema> {
    if one_symbol_schemata.len() <= 1 {
        return vec![TwoSymbolSchema::trivial(
            one_symbol_schemata.clone(),
            Some(signature),
            None,
        )];
    }
    // the members of the one_symbol_schema are hashed so that we can easily
    // check whether a permutation of a schema maintains closure
    let one_symbol_schemata_hash: HashSet<Vec<u8>> =
        HashSet::<_>::from_iter(one_symbol_schemata.clone());

    // Find the nontrivial columns of the one_symbol_schema; trivial columns are
    // those for which all symbols are the same
    let n_cols = one_symbol_schemata[0].len();
    let nontrivial_columns: Vec<usize> = (0..n_cols)
        .filter(|i| {
            one_symbol_schemata
                .iter()
                .any(|x| x[*i] != one_symbol_schemata[0][*i])
        })
        .collect();

    // initialize the two-symbol schema vector that we will eventually return
    let mut sym: HashSet<TwoSymbolSchema> = HashSet::new();

    // every one-symbol schemata must eventually be covered by a two symbol
    // schemata
    for root in one_symbol_schemata {
        // the schema `root` corresponds to the representative of the two-symbol
        // schemata (orbit equivalence class) that will generate on this
        // iteration of the loop.

        let swap_candidates = swap_candidates(root, one_symbol_schemata, &nontrivial_columns);

        // We iterate over all combinations of swaps, from most to least to find
        // the inclusion-maximal ones that work
        let mut good_swaps: Vec<HashSet<Vec<usize>>> = Vec::new();
        for skipped_swaps in swap_candidates.iter().powerset() {
            let mut swaps = swap_candidates.clone();
            swaps.retain(|x| !skipped_swaps.contains(&x));

            if good_swaps.iter().any(|x| swaps.is_subset(x)) {
                continue;
            }

            let (closed, redescribed_schemata, merged_swaps) =
                apply_action(root, &swaps, &one_symbol_schemata_hash);

            // if we leave the input set, the set of swaps does not form a group
            // action on our input set of one-symbol schemata
            if !closed || !redescribed_schemata.is_subset(&one_symbol_schemata_hash) {
                continue;
            }

            // record the columns of the redescribed schemata that are not the
            // same in this subset
            let trivial_redescription_columns: Vec<usize> = (0..n_cols)
                .filter(|i| redescribed_schemata.iter().all(|x| x[*i] == root[*i]))
                .collect();

            // we skip the swaps that include trivial columns because these are
            // not faithful group actions
            if swaps.iter().any(|swap| {
                swap.iter()
                    .any(|x| trivial_redescription_columns.contains(x))
            }) {
                continue;
            }

            // if we found a good swap, record it so that we don't spend
            // resources on its subsets
            good_swaps.push(swaps.clone());

            let bubble_indices =
                merged_swaps_to_bubbles(&merged_swaps, &trivial_redescription_columns);

            sym.insert(TwoSymbolSchema {
                redescribed_schemata: redescribed_schemata.iter().cloned().sorted().collect(),
                bubble_indices: bubble_indices.iter().cloned().sorted().collect(),
                signature: signature.clone(),
            });
        }
    }

    sym.into_iter().collect()
}

/// Find the largest symbol in all input pis.
fn compute_max_symbol(pis: &Vec<Vec<u8>>) -> usize {
    let mut max_symbol: u8 = 0;
    for pi in pis {
        for x in pi {
            if x > &max_symbol {
                max_symbol = *x;
            }
        }
    }
    max_symbol as usize
}

/// Find the indices where the input arrays `x` and `y` differ, and return a
/// vector of the indices. Optionally, a `break_above parameter` can be
/// provided, which returns early if the number of indices exceeds
/// `break_above`. This is useful for when we only want to identify arrays that
/// differ by a specific number of entries.
fn differing_indices(x: &[u8], y: &[u8], break_above: Option<usize>) -> Vec<usize> {
    let mut diff: Vec<usize> = Vec::new();
    let break_above = break_above.unwrap_or(x.len() + 1);
    let mut num_pushed: usize = 0;
    for (i, (a, b)) in x.iter().zip(y.iter()).enumerate() {
        if a != b {
            diff.push(i);
            num_pushed += 1;
            if num_pushed > break_above {
                break;
            }
        }
    }
    diff
}

/// Compute the signature of the one-symbol schema, which is the number of
/// instances of each unique symbol.
fn schema_signature(one_symbol_schema: &[u8], max_symbol: usize) -> Vec<usize> {
    let mut signature = vec![0; max_symbol + 1];
    for x in one_symbol_schema {
        signature[*x as usize] += 1;
    }
    signature
}

fn swap_candidates(
    root: &[u8],
    one_symbol_schemata: &[Vec<u8>],
    nontrivial_columns: &[usize],
) -> HashSet<Vec<usize>> {
    // A transposition is a candidate if 1. or 2. hold:
    // 1. it maps root to a different element of the one_symbol_schema
    let mut swap_candidates: HashSet<Vec<usize>> = one_symbol_schemata
        .iter()
        .map(|x| differing_indices(root, x, Some(2)))
        .filter(|y| y.len() == 2)
        .collect();
    // 2. it transposes nontrivial columns that leave root invariant.
    for pair in nontrivial_columns.iter().combinations(2) {
        let i = *pair[0];
        let j = *pair[1];
        if root[i] == root[j] {
            swap_candidates.insert(vec![i, j]);
        }
    }
    swap_candidates
}

/// Apply the group action described by `swaps` on `root`.
/// ### Returns: (`closed`, `redescribed_schemata`, `merged_swaps`)
/// `closed`: true iff the group action is closed <br>
/// `merged_swaps`: encodes permutations generated by swaps <br>
/// `redescribed_schemata`: the orbit of the action through root <br>
fn apply_action(
    root: &Vec<u8>,
    swaps: &HashSet<Vec<usize>>,
    one_symbol_schemata_hash: &HashSet<Vec<u8>>,
) -> (bool, HashSet<Vec<u8>>, Vec<HashSet<usize>>) {
    let mut redescribed_schemata: HashSet<Vec<u8>> = HashSet::new();
    redescribed_schemata.insert(root.clone());
    let mut merged_swaps: Vec<HashSet<usize>> =
        (0..root.len()).map(|ind| HashSet::from([ind])).collect();
    // This section applies the transfomrations to the root iteratively until no
    // new schemata are reached
    let mut old_size = 0;
    let mut closed = true;
    while old_size != redescribed_schemata.len() {
        old_size = redescribed_schemata.len();
        for swap in swaps.iter() {
            merged_swaps[swap[0]].insert(swap[1]);
            merged_swaps[swap[1]].insert(swap[0]);
            let new_schemata: HashSet<Vec<u8>> = redescribed_schemata
                .iter()
                .map(|g| {
                    let mut gs = g.clone();
                    gs.swap(swap[0], swap[1]);
                    gs
                })
                .collect();
            redescribed_schemata.extend(new_schemata.iter().cloned());
        }
        if !redescribed_schemata.is_subset(one_symbol_schemata_hash) {
            closed = false;
            break;
        }
    }
    (closed, redescribed_schemata, merged_swaps)
}

/// Convert the transpoitions encoded in `merged_swaps` to bubble indices,
/// filtering out trivial columns.
fn merged_swaps_to_bubbles(
    merged_swaps: &[HashSet<usize>],
    trivial_redescription_columns: &[usize],
) -> Vec<Vec<usize>> {
    // finally, convert the transpoitions to bubble indices
    let mut bubble_indices: Vec<Vec<usize>> = vec![];
    let mut seen_inds: HashSet<usize> = HashSet::new();
    for (i, x) in merged_swaps.iter().enumerate() {
        if seen_inds.contains(&i) {
            continue;
        }

        // only consider transpositions that map to nontrivial columns, i.e., we
        // are not doing same-symbol symmetry here
        if x.iter()
            .all(|&x| trivial_redescription_columns.contains(&x))
        {
            continue;
        }
        seen_inds.extend(x);

        let mut xv: Vec<usize> = x.iter().copied().collect();
        if xv.len() > 1 {
            // we use unstable sort because we don't have duplicates, and even
            // if we did, we wouldn't care if they got swapped
            xv.sort_unstable();
            bubble_indices.push(xv);
        }
    }
    bubble_indices
}
