use itertools::Itertools;
use pyo3::prelude::*;
use std::collections::{BTreeSet, HashMap, HashSet};

/// A Python module implemented in Rust.
#[pymodule]
fn schematodes(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(schemer, m)?)?;
    m.add_class::<TwoSymbolSchema>()?;
    Ok(())
}
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
#[pyclass]
struct TwoSymbolSchema {
    redescribed_schema: Vec<Vec<u8>>,
    bubble_indices: Vec<Vec<usize>>,
    signature: (usize, usize, usize),
}

#[pymethods]
impl TwoSymbolSchema {
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

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct OneSymbolSubset {
    schema: Vec<Vec<u8>>,
    indices: Vec<usize>,
    last_index_removed: Option<usize>,
}

#[pyfunction]
fn schemer(pis: Vec<Vec<u8>>) -> PyResult<Vec<TwoSymbolSchema>> {
    let mut tss_vec: Vec<TwoSymbolSchema> = Vec::new();

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

    for (signature, pi) in schema_with_signature {
        let tss = tss_for_group_with_signature(&pi, signature);
        tss_vec.extend(tss);
    }
    Ok(tss_vec)
}

fn tss_for_group_with_signature(
    group: &Vec<Vec<u8>>,
    signature: (usize, usize, usize),
) -> Vec<TwoSymbolSchema> {
    if group.len() <= 1 {
        assert!(group.len() == 1);
        let lone_schema = TwoSymbolSchema {
            redescribed_schema: group.to_vec(),
            bubble_indices: vec![],
            signature,
        };
        return vec![lone_schema];
    }

    let group_hash: HashSet<&Vec<u8>> = HashSet::<_>::from_iter(group);
    let n_cols = group[0].len();
    let nontrivial_columns: Vec<usize> = (0..n_cols)
        .filter(|i| group.iter().any(|x| x[*i] != group[0][*i]))
        .collect();

    let mut uncovered_schema: BTreeSet<&Vec<u8>> = BTreeSet::from_iter(group);

    let mut sym: Vec<TwoSymbolSchema> = vec![];

    while !uncovered_schema.is_empty() {
        let root = uncovered_schema.pop_last().unwrap();
        let mut swap_candidates: Vec<Vec<usize>> = group
            .iter()
            .map(|x| differing_indices(root, x, Some(2)))
            .filter(|y| y.len() == 2)
            .collect();

        for pair in nontrivial_columns.iter().combinations(2) {
            let i = *pair[0];
            let j = *pair[1];
            if root[i] == root[j] {
                swap_candidates.push(vec![i, j]);
            }
        }

        let rep = root.clone();

        let mut redescribed_schema: HashSet<Vec<u8>> = HashSet::new();
        redescribed_schema.insert(rep);
        let mut merged_swaps: Vec<HashSet<usize>> = (0..root.len())
            .map(|ind| HashSet::from_iter(ind..ind + 1))
            .collect();
        for y in swap_candidates {
            let swapped_schema: HashSet<Vec<u8>> = redescribed_schema
                .iter()
                .map(|g| {
                    let mut gs = g.clone();
                    gs.swap(y[0], y[1]);
                    gs
                })
                .collect();

            if swapped_schema.iter().all(|g| group_hash.contains(&g)) {
                redescribed_schema.extend(swapped_schema);
                uncovered_schema.retain(|&g| !redescribed_schema.contains(g));
                merged_swaps[y[0]].insert(y[1]);
                merged_swaps[y[1]].insert(y[0]);
            }
        }
        let nontrivial_redescription_columns: Vec<usize> = (0..n_cols)
            .filter(|i| redescribed_schema.iter().any(|x| x[*i] != root[*i]))
            .collect();
        // nontrivial_redescription_columns.sort();
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
        sym.push(TwoSymbolSchema {
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
