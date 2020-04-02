mod read_fasta;
mod read_result_csv;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;


#[pyfunction]
fn hello() -> PyResult<()> {
    println!("Hello, world!");
    Ok(())
}

#[pyfunction]
fn read_fasta(path: &str) -> PyResult<HashMap<String, String>> {
    let id2seq = read_fasta::read_fasta_core(&path);
    Ok(id2seq.unwrap())
}

#[pyfunction]
fn read_result_csv(path: &str, key: usize) -> PyResult<HashMap<String, HashMap<String, String>>> {
    let result_csv = read_result_csv::parse_result_csv_core(&path, key);
    Ok(result_csv.unwrap())
}

#[pymodule]
fn my_parser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!( hello ))?;
    m.add_wrapped(wrap_pyfunction!( read_fasta ))?;
    m.add_wrapped(wrap_pyfunction!( read_result_csv ))?;
    Ok(())
}