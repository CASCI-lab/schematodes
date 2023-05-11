use itertools::Itertools;
use pyo3::prelude::*;
use std::collections::{BTreeSet, HashMap, HashSet};

/// A Python module implemented in Rust.
#[pymodule]
fn schematodes(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(schemer, m)?)?;
    m.add_class::<TwoSymbolSchemata>()?;
    Ok(())
}

/// A Python class implemented in Rust. This is the primary return type of the module. See stub file for Python usage.
/// The class contains three fields: redescribed_schema, bubble_indices, and signature. These are the set of one-symbol schemata that are redescribed,
/// the indices of the bubbles, and the signature (number of 0s, number of 1s, and number of #s) of the schema.
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
#[pyclass]
struct TwoSymbolSchemata {
    redescribed_schema: Vec<Vec<u8>>,
    bubble_indices: Vec<Vec<usize>>,
    signature: (usize, usize, usize),
}

#[pymethods]
impl TwoSymbolSchemata {
    #[new]
    fn py_new(redescribed_schema: Vec<Vec<u8>>, bubble_indices: Vec<Vec<usize>>) -> PyResult<Self> {
        Ok(Self {
            redescribed_schema: redescribed_schema.clone(),
            bubble_indices,
            signature: compute_signature(&redescribed_schema[0]),
        })
    }
    #[getter]
    fn get_redescribed_schema(&self) -> PyResult<Vec<Vec<u8>>> {
        Ok(self.redescribed_schema.clone())
    }
    #[getter]
    fn get_bubble_indices(&self) -> PyResult<Vec<Vec<usize>>> {
        Ok(self.bubble_indices.clone())
    }
}

/// This is the function that will be used in Python to redescribe a set of one-symbol schema as a list of two-symbol schema.
#[pyfunction]
fn schemer(pis: Vec<Vec<u8>>) -> PyResult<Vec<TwoSymbolSchemata>> {
    let mut tss_vec: Vec<TwoSymbolSchemata> = Vec::new();

    // gather one-symbol schema by the number of 0s, 1s, and #s in the schema.
    let mut schema_with_signature: HashMap<(usize, usize, usize), Vec<Vec<u8>>> = HashMap::new();
    for pi in pis {
        let signature = compute_signature(&pi);
        if !schema_with_signature.contains_key(&signature) {
            schema_with_signature.insert(signature, vec![pi]);
        } else {
            let mut pi_vec: Vec<Vec<u8>> = schema_with_signature[&signature].to_vec();
            pi_vec.push(pi);
            schema_with_signature.insert(signature, pi_vec);
        }
    }

    // Loop through the unique signatures and compress the corresponding schema for each.
    for (signature, pi) in schema_with_signature {
        let tss = tss_for_one_symbol_schema_with_signature(&pi, signature);
        tss_vec.extend(tss);
    }
    Ok(tss_vec)
}

/// Compress a one_symbol_schema of schema that have the same signature.
/// This function does not verify that the signatures are equal, and will give incorrect results if they are not.
/// Returns a vector of TwoSymbolSchemata objects corresponding to a one_symbol_schema action of a product of symmetric one_symbol_schemas.
fn tss_for_one_symbol_schema_with_signature(
    one_symbol_schema: &Vec<Vec<u8>>,
    signature: (usize, usize, usize),
) -> Vec<TwoSymbolSchemata> {
    if one_symbol_schema.len() <= 1 {
        assert!(one_symbol_schema.len() == 1);
        let lone_schema = TwoSymbolSchemata {
            redescribed_schema: one_symbol_schema.to_vec(),
            bubble_indices: vec![],
            signature,
        };
        return vec![lone_schema];
    }
    // the members of the one_symbol_schema are hashed so that we can easily check whether a permutation of a schema maintains closure
    let one_symbol_schema_hash: HashSet<Vec<u8>> =
        HashSet::<_>::from_iter(one_symbol_schema.clone());
    // Find the nontrivial columns of the one_symbol_schema; trivial columns are those for which all symbols are the same
    let n_cols = one_symbol_schema[0].len();
    let nontrivial_columns: Vec<usize> = (0..n_cols)
        .filter(|i| {
            one_symbol_schema
                .iter()
                .any(|x| x[*i] != one_symbol_schema[0][*i])
        })
        .collect();

    // initialize the two-symbol schema vector that we will eventually return
    let mut sym: HashSet<TwoSymbolSchemata> = HashSet::new();

    // every one-symbol schemata must eventually be covered by a two symbol schemata
    let mut uncovered_schema: BTreeSet<&Vec<u8>> = BTreeSet::from_iter(one_symbol_schema);
    while !uncovered_schema.is_empty() {
        // the schemata `root` corresponds to the representative of the two-symbol schemata that will generate on this iteration of the loop
        let root = uncovered_schema.pop_last().unwrap();
        // a transposition is a candidate if
        // 1. it maps root to a different element of the one_symbol_schema OR
        // 2. it transposes nontrivial columns that leave root invariant.
        // condition 1:
        let mut swap_candidates: HashSet<Vec<usize>> = one_symbol_schema
            .iter()
            .map(|x| differing_indices(root, x, Some(2)))
            .filter(|y| y.len() == 2)
            .collect();
        // condition 2:
        for pair in nontrivial_columns.iter().combinations(2) {
            let i = *pair[0];
            let j = *pair[1];
            if root[i] == root[j] {
                swap_candidates.insert(vec![i, j]);
            }
        }
        let mut good_swaps: Vec<HashSet<Vec<usize>>> = Vec::new();
        for skipped_swaps in swap_candidates.iter().powerset() {
            // let mut transpositions: Vec<Vec<usize>> = Vec::new();
            let mut swaps = swap_candidates.clone();
            swaps.retain(|x| !skipped_swaps.contains(&x));

            if good_swaps.iter().any(|x| swaps.is_subset(x)) {
                continue;
            }

            // Now we start looking for an inclusion-maximal product of symmetric groups that can act on root while maintaining closure.
            // The trivial group acting on root is the trivial case; we will expand from there.
            let mut redescribed_schema: HashSet<Vec<u8>> = HashSet::new();
            redescribed_schema.insert(root.clone());
            let mut merged_swaps: Vec<HashSet<usize>> = (0..root.len())
                .map(|ind| HashSet::from_iter(ind..ind + 1))
                .collect();

            let mut old_size = 0;
            let mut closed = true;
            while old_size != redescribed_schema.len() {
                old_size = redescribed_schema.len();
                for swap in swaps.iter() {
                    merged_swaps[swap[0]].insert(swap[1]);
                    merged_swaps[swap[1]].insert(swap[0]);
                    let new_schema: HashSet<Vec<u8>> = redescribed_schema
                        .iter()
                        .map(|g| {
                            let mut gs = g.clone();
                            gs.swap(swap[0], swap[1]);
                            gs
                        })
                        .collect();
                    redescribed_schema.extend(new_schema.iter().cloned());
                }
                if !redescribed_schema.is_subset(&one_symbol_schema_hash) {
                    closed = false;
                    break;
                }
            }
            if !closed || !redescribed_schema.is_subset(&one_symbol_schema_hash) {
                continue;
            }
            println!("root = {:?}", root);
            println!("swaps = {:?}; good_swaps = {:?}", swaps, good_swaps);
            good_swaps.push(swaps.clone());
            // // Iterate through the candidate swaps and merge them in order if they keeps the merged set in the one_symbol_schema set.
            // for y in swaps.iter() {
            //     // find the orbit of the transpositions so far and the candiated transposition y

            //     let mut transpotion_candidates: Vec<Vec<usize>> = transpositions.clone();
            //     transpotion_candidates.push(y.clone());
            //     let mut closure_found = false;
            //     while !closure_found {
            //         let old_size = swapped_schema.len();
            //         for transposition in transpotion_candidates.iter() {
            //             let transposed_schema: HashSet<Vec<u8>> = swapped_schema
            //                 .iter()
            //                 .map(|g| {
            //                     let mut gs = g.clone();
            //                     gs.swap(transposition[0], transposition[1]);
            //                     gs
            //                 })
            //                 .collect();
            //             swapped_schema.extend(transposed_schema);
            //         }
            //         closure_found = old_size == swapped_schema.len();
            //     }

            //     // check if the swap maps redescribed_schema into the one_symbol_schema set;
            //     // if so, add this image to the redscribed_schema and record the swapped indices.
            //     if swapped_schema
            //         .iter()
            //         .all(|g| one_symbol_schema_hash.contains(&g))
            //     {
            //         redescribed_schema.extend(swapped_schema);
            //         // uncovered_schema.retain(|&g| !redescribed_schema.contains(g));
            //         transpositions.push(y.clone());
            //         merged_swaps[y[0]].insert(y[1]);
            //         merged_swaps[y[1]].insert(y[0]);
            //         good_swaps.push(swaps.clone());
            //     }
            // }
            // record the columns of the redescribed schema that are not the same in this subset
            let nontrivial_redescription_columns: Vec<usize> = (0..n_cols)
                .filter(|i| redescribed_schema.iter().any(|x| x[*i] != root[*i]))
                .collect();

            // finally, convert the transpoitions to bubble indices
            let mut bubble_indices: Vec<Vec<usize>> = vec![];
            let mut seen_inds: HashSet<usize> = HashSet::new();
            for (i, x) in merged_swaps.iter().enumerate() {
                if seen_inds.contains(&i) {
                    // avoid duplicates
                    continue;
                }
                if x.iter() // only consider transpositions that map to nontrivial columns, i.e., we are not doing same-symbol symmetry here
                    .any(|&x| !nontrivial_redescription_columns.contains(&x))
                {
                    continue;
                }
                seen_inds.extend(x);

                let mut xv: Vec<usize> = x.iter().cloned().collect(); // cana expects a list of bubble indices, so we convert to vec here
                if xv.len() > 1 {
                    xv.sort_unstable(); // we use unstable sort because we don't have duplicates, and even if we did, we wouldn't care if they got swapped
                    bubble_indices.push(xv);
                }
            }
            println!("merged_swaps = {:?}", merged_swaps);
            println!("bubble_indices = {:?}", bubble_indices);
            sym.insert(TwoSymbolSchemata {
                redescribed_schema: redescribed_schema
                    .iter()
                    .map(|x| x.to_vec())
                    .sorted()
                    .collect(),
                bubble_indices: bubble_indices.iter().map(|x| x.to_vec()).sorted().collect(),
                signature,
            });
        }
    }

    sym.into_iter().collect()
}

/// Permute x according to p
fn permute(x: &[u8], p: &[usize]) -> Vec<u8> {
    let mut y = x.to_vec();
    for (i, j) in p.iter().sorted().zip(p) {
        y.swap(*i, *j);
    }
    y
}

/// Find the indices where the input arrays `x` and `y` differ, and return a vector of the indices.
/// Optionally, a `break_above parameter` can be provided, which returns early if the number of indices exceeds `break_above`.
/// This is useful for when we only want to identify arrays that differ by a specific number of entries.
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

/// Compute the signature of the one-symbol schemata, which is the number of 0s, 1s, and 2s
fn compute_signature(one_symbol_schemata: &[u8]) -> (usize, usize, usize) {
    let mut signature = (0, 0, 0);
    for x in one_symbol_schemata {
        match x {
            0 => signature.0 += 1,
            1 => signature.1 += 1,
            2 => signature.2 += 1,
            _ => (),
        }
    }
    signature
}
