use pyo3::prelude::*;

#[pyfunction]
pub fn test_lib_code_fn() -> String {
    "Hello from test_lib_code!".to_string()
}

/// 2つの整数を受け取り、その和を返す関数
#[pyfunction]
pub fn test_lib_code_fn_2(a: i32, b: i32) -> String {
    format!("Hello from test_lib_code_2! {} {}", a, b)
}
