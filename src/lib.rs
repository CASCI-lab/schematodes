use pyo3::prelude::*;
use std::collections::{HashMap, HashSet};

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
    signature: usize,
}

#[pymethods]
impl TwoSymbolSchema {
    #[new]
    fn py_new(redescribed_schema: Vec<Vec<u8>>, bubble_indices: Vec<Vec<usize>>) -> PyResult<Self> {
        Ok(Self {
            redescribed_schema: redescribed_schema.clone(),
            bubble_indices,
            signature: redescribed_schema[0].iter().map(|x| *x as usize).sum(),
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

impl OneSymbolSubset {
    fn remove_index(&mut self, index_to_remove: usize) {
        if let Some(index_position) = self.indices.iter().position(|i| *i == index_to_remove) {
            self.schema.remove(index_position);
            self.indices.remove(index_position);
            self.last_index_removed = Some(index_to_remove);
        } else {
            panic!();
        }
    }
}

#[pyfunction]
fn schemer(pis: Vec<Vec<u8>>) -> PyResult<Vec<TwoSymbolSchema>> {
    let mut found_sym: HashSet<Vec<usize>> = HashSet::new();
    let mut tss_vec: Vec<TwoSymbolSchema> = Vec::new();
    let mut tss_signature_map: HashMap<usize, HashSet<TwoSymbolSchema>> = HashMap::new();
    let mut next_depth_queue: HashSet<OneSymbolSubset> = HashSet::new();
    let n_schema = pis.len();
    next_depth_queue.insert(OneSymbolSubset {
        schema: pis,
        indices: (0..n_schema).collect(),
        last_index_removed: None,
    });

    for _depth in 0..n_schema {
        let depth_queue = next_depth_queue.clone();
        next_depth_queue.clear();

        for oss in depth_queue {
            let group = oss.schema.clone();
            let included_inds = oss.indices.clone();
            // at this point, group is a vector of one symbol schemata, each represented as a vector of 0s, 1s, and 2s,
            // and included_inds is a vector of indices into pis that describe group.

            if combo_superset_seen(&included_inds, &found_sym) {
                continue;
            }

            // now, if the row sums are all the same, we can try to collapse the group
            let signatures: Vec<usize> = group
                .iter()
                .map(|g| g.iter().map(|x| *x as usize).sum())
                .collect();

            let equivalent_signatures: bool = signatures.iter().all(|g| *g == signatures[0]);
            let mut sym: Vec<Vec<usize>> = Vec::new();
            if equivalent_signatures {
                sym = check_for_symmetry(&group);
            }

            if group.len() == 1 || !sym.is_empty() {
                // symmetry to report or done
                found_sym.insert(included_inds.clone());
                let tss = TwoSymbolSchema {
                    redescribed_schema: group.iter().map(|x| x.to_vec()).collect(),
                    bubble_indices: sym,
                    signature: signatures[0],
                };
                tss_vec.push(tss.clone());
                let mut sigmap = tss_signature_map
                    .get(&signatures[0])
                    .unwrap_or(&HashSet::new() as &HashSet<TwoSymbolSchema>)
                    .clone();
                sigmap.insert(tss);
                tss_signature_map.insert(signatures[0], sigmap);
            } else {
                for index_to_remove in included_inds {
                    if let Some(last_index_removed) = oss.last_index_removed {
                        // avoids duplicating groups
                        if last_index_removed > index_to_remove {
                            continue;
                        }
                    }

                    next_depth_queue.insert({
                        let mut new_oss = oss.clone();
                        new_oss.remove_index(index_to_remove);
                        new_oss
                    });
                }
            }
        }
    }
    Ok(tss_vec)
}

fn collapse_two_symbols(tss_signature_map: &mut HashMap<usize, HashSet<TwoSymbolSchema>>) {
    for tss_vec in tss_signature_map.values_mut() {
        let n_tss = tss_vec.len();
        if n_tss < 2 {
            continue;
        }

        // loop through tss_vec via v
        // loop through tss_vec via u
        // ???
        // profit!
    }

    todo!()
}

fn merge_two_symbol_schemas(tss1: &TwoSymbolSchema, tss2: &TwoSymbolSchema) -> TwoSymbolSchema {
    let mut new_schema = tss1.redescribed_schema.clone();
    new_schema.extend(tss2.redescribed_schema.clone());
    let mut new_indices = tss1.bubble_indices.clone();
    new_indices.extend(tss2.bubble_indices.clone());
    TwoSymbolSchema {
        redescribed_schema: new_schema,
        bubble_indices: new_indices,
        signature: tss1.signature,
    }
}

fn combo_superset_seen(included_inds: &Vec<usize>, found_sym: &HashSet<Vec<usize>>) -> bool {
    for seen in found_sym {
        let mut seen_matches = true;
        for i in included_inds {
            if !seen.contains(i) {
                seen_matches = false;
                break;
            }
        }
        if seen_matches {
            return true;
        }
    }
    false
}

fn check_for_symmetry(group: &Vec<Vec<u8>>) -> Vec<Vec<usize>> {
    if group.len() <= 1 {
        return Vec::new();
    }

    let n_cols = group[0].len();
    let mut sym: Vec<Vec<usize>> = Vec::new();
    let nontrivial_columns: Vec<usize> = (0..n_cols)
        .filter(|i| group.iter().any(|x| x[*i] != group[0][*i]))
        .collect();

    let group_hash: HashSet<Vec<u8>> = group
        .iter()
        .map(|g| {
            let g_iter: Vec<u8> = g
                .iter()
                .enumerate()
                .filter(|(i, _)| nontrivial_columns.contains(i))
                .map(|(_, x)| *x)
                .collect();
            g_iter.to_vec()
        })
        .collect();

    let group_hash_swap_two: HashSet<Vec<u8>> = group_hash
        .iter()
        .cloned()
        .map(|x| {
            let mut y = x;
            y.swap(0, 1);
            y
        })
        .collect();

    let group_hash_full_cycle: HashSet<Vec<u8>> = group_hash
        .iter()
        .map(|x| {
            let mut y = Vec::from_iter(x[1..].iter().cloned());
            y.push(x[0]);
            y
        })
        .collect();

    if group_hash == group_hash_swap_two && group_hash == group_hash_full_cycle {
        sym = vec![nontrivial_columns]
    }

    sym
}
