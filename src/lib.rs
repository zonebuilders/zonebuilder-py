use zonebuilder;
use pyo3::prelude::*;

#[pyfunction]
fn triangular_sequence(n: usize) -> Vec<f64> {
    zonebuilder::triangular_sequence(n)
}

#[pymodule]
fn zonebuilder_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(triangular_sequence, m)?)?;
    Ok(())
}