use std::collections::HashSet;

use pyo3::prelude::*;
use itertools::Itertools;

/// A Python module implemented in Rust.
#[pymodule]
fn schematodes(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(schemer, m)?)?;
    m.add_class::<TwoSymbolSchema>()?;
    Ok(())
}

#[pyclass]
struct TwoSymbolSchema {
    redescribed_schema: Vec<Vec<u8>>,
    bubble_indices: Vec<Vec<usize>>,
}

#[pymethods]
impl TwoSymbolSchema {
    #[new]
    fn py_new(redescribed_schema: Vec<Vec<u8>>, bubble_indices: Vec<Vec<usize>>) -> PyResult<Self> {
        Ok(Self{redescribed_schema, bubble_indices})
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
fn schemer(pis: Vec<Vec<u8>>) -> PyResult<Vec<TwoSymbolSchema>> {
    let mut found_sym: HashSet<Vec<usize>> = HashSet::new();
    let mut tss_vec: Vec<TwoSymbolSchema> = Vec::new();
    for depth in 0..pis.len() {
        let combo_indices = (0..pis.len()).combinations(pis.len()-depth);
        let combo_group = pis.iter().combinations(pis.len()-depth);
        
        for (group, included_inds) in combo_group.zip(combo_indices) {
            println!("group = {:?}",&group);
            if combon_superset_seen(&included_inds, &found_sym){
                continue
            }
            // at this point, group is a vector of one symbol schemata, each represented as a vector of 0s, 1s, and 2s.
            let sym = check_for_symmetry(&group);
            println!("sym = {:?}",&sym);
            if group.len() == 1 || !sym.is_empty() {
                found_sym.insert(included_inds);
                tss_vec.push(TwoSymbolSchema {
                    redescribed_schema: group.iter().map(|x| x.to_vec()).collect(),
                    bubble_indices: sym,
                })
            }
        }
    }
    Ok(tss_vec)
}

fn combon_superset_seen(included_inds: &Vec<usize>, found_sym: &HashSet<Vec<usize>>) -> bool {
    for seen in found_sym {
        let mut seen_matches = true;
        for i in included_inds {
            if !seen.contains(i){
                seen_matches = false;
                break
            }
        }
        if seen_matches {
            return true;
        }
    }
    false
}

fn check_for_symmetry(group: &[&Vec<u8>]) -> Vec<Vec<usize>> {
    if group.len() <= 1 {
        return Vec::new();
    }

    let n_cols = group[0].len();
    let mut sym: Vec<Vec<usize>> = Vec::new();
    let nontrivial_columns: Vec<usize> = (0..n_cols).filter( |i|group.iter().any(|x| x[*i] != group[0][*i])).collect();
    
    let group_hash: HashSet<Vec<u8>> = group
        .iter()
        .map(|g| {
            let g_iter: Vec<u8> = g
            .iter().enumerate()
            .filter(|(i,_)| nontrivial_columns.contains(i)) 
            .map( |(_,x)| *x)
            .collect();
            g_iter.to_vec()

        }
        )
        .collect();
    
    let group_hash_swap_two: HashSet<Vec<u8>> = group_hash
        .iter()
        .cloned()
        .map(|x| {
            let mut y = x;
            y.swap(0,1);
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
    println!("group input = {:?}",&group);
    println!("nontrivial_columns = {:?}",&nontrivial_columns);
    println!("group_hash = {:?}",&group_hash);
    println!("group_hash_swap_two = {:?}",&group_hash_swap_two);
    println!("group_hash_full_cycle = {:?}",&group_hash_full_cycle);
    if group_hash == group_hash_swap_two && group_hash == group_hash_full_cycle{
        sym = vec![nontrivial_columns]
    }



    sym
}