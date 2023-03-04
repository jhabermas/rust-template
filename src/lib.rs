pub mod util;

use pyo3::prelude::*;

#[pyfunction]
fn get_fibonacci_rust(number: isize) -> PyResult<u128> {
    Ok(util::get_fibonacci(number))
}

#[pymodule]
fn hello(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_fibonacci_rust, m)?)?;
    Ok(())
}