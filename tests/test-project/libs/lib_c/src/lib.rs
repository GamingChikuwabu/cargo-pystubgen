use pyo3::prelude::*;

#[pyfunction]
fn hello_from_bin() -> String {
    "Hello from lib-c!".to_string()
}

#[pyfunction]
fn hello_from_bin_2(a: i32, b: i32) -> String {
    format!("Hello from lib-c! {} {}", a, b)
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_from_bin, m)?)?;
    m.add_function(wrap_pyfunction!(hello_from_bin_2, m)?)?;
    Ok(())
}
