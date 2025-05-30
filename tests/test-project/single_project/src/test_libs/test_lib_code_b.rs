use pyo3::prelude::*;

#[pyfunction]
pub fn test_lib_code_b_fn() -> String {
    "Hello from test_lib_code_b!".to_string()
}


#[pyfunction]
pub fn test_lib_code_b_fn_2() -> String {
    "Hello from test_lib_code_b_2!".to_string()
}
