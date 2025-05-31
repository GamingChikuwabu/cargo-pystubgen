use pyo3::prelude::*;

mod test_libs;


#[pyfunction]
fn hello_from_bin() -> String {
    "Hello from single-project!".to_string()
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_from_bin, m)?)?;
    m.add_function(wrap_pyfunction!(test_libs::test_lib_code::test_lib_code_fn, m)?)?;
    m.add_function(wrap_pyfunction!(test_libs::test_lib_code::test_lib_code_fn_2, m)?)?;
    m.add_function(wrap_pyfunction!(test_libs::test_lib_code::test_numeric_types, m)?)?;
    m.add_function(wrap_pyfunction!(test_libs::test_lib_code::test_string_types, m)?)?;
    m.add_function(wrap_pyfunction!(test_libs::test_lib_code::test_add_two_numbers, m)?)?;
    m.add_function(wrap_pyfunction!(test_libs::test_lib_code::test_collection_types, m)?)?;
    m.add_function(wrap_pyfunction!(test_libs::test_lib_code::test_tuple_types, m)?)?;
    m.add_function(wrap_pyfunction!(test_libs::test_lib_code::test_hashmap_types, m)?)?;
    m.add_function(wrap_pyfunction!(test_libs::test_lib_code::test_option_types, m)?)?;
    m.add_function(wrap_pyfunction!(test_libs::test_lib_code_b::test_lib_code_b_fn, m)?)?;
    m.add_function(wrap_pyfunction!(test_libs::test_lib_code_b::test_lib_code_b_fn_2, m)?)?; 

    Ok(())
}
