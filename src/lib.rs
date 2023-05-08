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

/// A Python class implemented in Rust.
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
    let one_symbol_schema_hash: HashSet<&Vec<u8>> = HashSet::<_>::from_iter(one_symbol_schema);

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
    let mut sym: Vec<TwoSymbolSchemata> = vec![];

    // every one-symbol schemata must eventually be covered by a two symbol schemata
    let mut uncovered_schema: BTreeSet<&Vec<u8>> = BTreeSet::from_iter(one_symbol_schema);
    while !uncovered_schema.is_empty() {
        // the schemata `root` corresponds to the representative of the two-symbol schemata that will generate on this iteration of the loop
        let root = uncovered_schema.pop_last().unwrap();

        // a transposition is a candidate if
        // 1. it maps root to a different element of the one_symbol_schema OR
        // 2. it transposes nontrivial columns that leave root invariant.
        // condition 1:
        let mut swap_candidates: Vec<Vec<usize>> = one_symbol_schema
            .iter()
            .map(|x| differing_indices(root, x, Some(2)))
            .filter(|y| y.len() == 2)
            .collect();
        // condition 2:
        for pair in nontrivial_columns.iter().combinations(2) {
            let i = *pair[0];
            let j = *pair[1];
            if root[i] == root[j] {
                swap_candidates.push(vec![i, j]);
            }
        }

        // Now we start looking for an inclusion-maximal product of symmetric groups that can act on root while maintaining closure.
        // The trivial group acting on root is the trivial case; we will expand from there.
        let mut redescribed_schema: HashSet<Vec<u8>> = HashSet::new();
        redescribed_schema.insert(root.clone());
        let mut merged_swaps: Vec<HashSet<usize>> = (0..root.len())
            .map(|ind| HashSet::from_iter(ind..ind + 1))
            .collect();
        // Iterate through the candidate swaps and merge them in order if they keeps the merged set in the one_symbol_schema set.
        for y in swap_candidates {
            // this iter,map,collect chain applies the swap to everything we have added so far
            let swapped_schema: HashSet<Vec<u8>> = redescribed_schema
                .iter()
                .map(|g| {
                    let mut gs = g.clone();
                    gs.swap(y[0], y[1]);
                    gs
                })
                .collect();

            // check if the swap maps redescribed_schema into the one_symbol_schema set;
            // if so, add this image to the redscribed_schema and record the swapped indices.
            if swapped_schema
                .iter()
                .all(|g| one_symbol_schema_hash.contains(&g))
            {
                redescribed_schema.extend(swapped_schema);
                uncovered_schema.retain(|&g| !redescribed_schema.contains(g));
                merged_swaps[y[0]].insert(y[1]);
                merged_swaps[y[1]].insert(y[0]);
            }
        }
        // record the columns of the redescribed schema that are not the same in this subset
        let nontrivial_redescription_columns: Vec<usize> = (0..n_cols)
            .filter(|i| redescribed_schema.iter().any(|x| x[*i] != root[*i]))
            .collect();

        // finally, convert the transpoitions to bubble indices
        let mut bubble_indices: Vec<Vec<usize>> = vec![];
        let mut seen_inds: HashSet<usize> = HashSet::new();
        for (i, x) in merged_swaps.iter().enumerate() {
            if seen_inds.contains(&i) {
                continue;
            }
            if x.iter()
                .any(|&x| !nontrivial_redescription_columns.contains(&x))
            {
                continue;
            }
            seen_inds.extend(x);
            let mut xv: Vec<usize> = x.iter().cloned().collect();
            if xv.len() > 1 {
                xv.sort_unstable();
                bubble_indices.push(xv);
            }
        }
        sym.push(TwoSymbolSchemata {
            redescribed_schema: redescribed_schema.iter().map(|x| x.to_vec()).collect(),
            bubble_indices,
            signature,
        });
    }

    sym
}

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
