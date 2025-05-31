use pyo3::prelude::*;
use std::collections::{HashMap, HashSet};

#[pyfunction]
pub fn test_lib_code_fn() -> String {
    "Hello from test_lib_code!".to_string()
}

#[pyfunction]
pub fn test_add_two_numbers(a: i32, b: i32){
    println!("{}", a + b);
}

/// 2つの整数を受け取り、その和を返す関数
#[pyfunction]
pub fn test_lib_code_fn_2(a: i32, b: i32) -> String {
    format!("Hello from test_lib_code_2! {} {}", a, b)
}

/// 基本的な数値型を使用した関数
#[pyfunction]
pub fn test_numeric_types(
    int_val: i32,
    float_val: f64,
    unsigned_val: u64,
) -> PyResult<(i32, f64, u64)> {
    Ok((int_val * 2, float_val * 2.0, unsigned_val * 2))
}

/// 文字列と文字列スライスを使用した関数
#[pyfunction]
pub fn test_string_types(text: &str) -> String {
    format!("入力された文字列: {}", text)
}

/// 配列とベクターを使用した関数
#[pyfunction]
pub fn test_collection_types(
    numbers: Vec<i32>,
    text_list: Vec<String>,
) -> PyResult<(Vec<i32>, Vec<String>)> {
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    let uppercased: Vec<String> = text_list.iter().map(|s| s.to_uppercase()).collect();
    Ok((doubled, uppercased))
}

/// タプルを使用した関数
#[pyfunction]
pub fn test_tuple_types(
    tuple: (i32, String, f64),
) -> (i32, String, f64) {
    let (num, text, float) = tuple;
    (num * 2, format!("{}_modified", text), float * 2.0)
}

/// ハッシュマップを使用した関数
#[pyfunction]
pub fn test_hashmap_types(
    map: HashMap<String, i32>,
) -> PyResult<HashMap<String, i32>> {
    let mut result = HashMap::new();
    for (key, value) in map {
        result.insert(format!("new_{}", key), value * 2);
    }
    Ok(result)
}

/// オプション型を使用した関数
#[pyfunction]
#[pyo3(signature = (maybe_number=None, maybe_text=None))]
pub fn test_option_types(
    maybe_number: Option<i32>,
    maybe_text: Option<String>,
) -> (Option<i32>, Option<String>) {
    (
        maybe_number.map(|n| n * 2),
        maybe_text.map(|s| format!("{}_modified", s)),
    )
}

/// カスタム構造体を使用した関数
#[pyclass]
#[derive(Clone)]
struct TestStruct {
    #[pyo3(get)]
    number: i32,
    #[pyo3(get)]
    text: String,
}

#[pymethods]
impl TestStruct {
    #[new]
    fn new(number: i32, text: String) -> Self {
        TestStruct { number, text }
    }

    fn to_string(&self) -> String {
        format!("TestStruct(number={}, text={})", self.number, self.text)
    }
}

#[pyfunction]
pub fn test_custom_struct(struct_instance: TestStruct) -> TestStruct {
    TestStruct {
        number: struct_instance.number * 2,
        text: format!("{}_modified", struct_instance.text),
    }
}
