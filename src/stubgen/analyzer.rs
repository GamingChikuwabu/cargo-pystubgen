//! Rustコード解析モジュール
//! 
//! このモジュールは、パースされたRustコードをPythonの型情報に変換します。
//! Rustの型システムからPythonの型ヒントへの変換を行います。

use crate::stubgen::parser::*;
use crate::stubgen::typemap::*;

/// Python関数の型情報を保持する構造体
#[derive(Debug)]
pub struct PythonFunctionData {
    /// 関数名
    pub name: String,
    /// 引数のリスト（名前と型のペア）
    pub args: Vec<(String, String)>,
    /// 戻り値の型
    pub return_type: String,
    /// ドキュメントコメント
    pub doc: String,
}

/// Pythonモジュールの型情報を保持する構造体
#[derive(Debug)]
pub struct PythonSrcData {
    /// 関数のリスト
    pub functions: Vec<PythonFunctionData>,
}

/// Rustソースコードの型情報をPythonの型情報に変換する
/// 
/// # Arguments
/// 
/// * `rust_src_data` - パースされたRustソースコードの型情報
/// 
/// # Returns
/// 
/// * `PythonSrcData` - 変換されたPythonの型情報
pub fn analyze_rust_src_data(rust_src_data: &RustSrcData) -> PythonSrcData {
    let functions = rust_src_data.functions
        .iter()
        .map(analyze_function_data)
        .collect();
    
    PythonSrcData { functions }
}

/// 個々のRust関数の型情報をPythonの型情報に変換する
/// 
/// # Arguments
/// 
/// * `function_data` - パースされたRust関数の型情報
/// 
/// # Returns
/// 
/// * `PythonFunctionData` - 変換されたPython関数の型情報
pub fn analyze_function_data(function_data: &RustFunctionData) -> PythonFunctionData {
    PythonFunctionData {
        name: analyze_function_name(function_data),
        args: analyze_function_args(function_data),
        return_type: analyze_function_return_type(function_data),
        doc: analyze_function_doc(function_data),
    }
}

/// 関数名を取得する
fn analyze_function_name(function_data: &RustFunctionData) -> String {
    function_data.name.clone()
}

/// 関数の引数リストをPythonの型情報に変換する
fn analyze_function_args(function_data: &RustFunctionData) -> Vec<(String, String)> {
    function_data.args
        .iter()
        .map(|(name, ty)| {
            let rust_type = syn::parse_str::<syn::Type>(ty)
                .expect("Failed to parse Rust type");
            let python_type = map_type(&rust_type);
            (name.clone(), python_type)
        })
        .collect()
}

/// 関数の戻り値の型をPythonの型情報に変換する
fn analyze_function_return_type(function_data: &RustFunctionData) -> String {
    let rust_type = syn::parse_str::<syn::Type>(&function_data.return_type)
        .expect("Failed to parse Rust return type");
    map_type(&rust_type)
}

/// 関数のドキュメントコメントを取得する
fn analyze_function_doc(function_data: &RustFunctionData) -> String {
    function_data.doc.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CODE: &str = r#"
    /// testcode add function
    /// second line
    #[pyfunction]
    #[testattribute]
    fn add(a: i32, b: i32) -> Vec<i32> {
        vec![a + b]
    }
    "#;

    #[test]
    fn test_analyze_function_name() {
        let function_data = parse_function_data(&syn::parse_str(TEST_CODE).unwrap());
        let python_function_data = analyze_function_data(&function_data);
        assert_eq!(python_function_data.name, "add");
    }

    #[test]
    fn test_analyze_function_args() {
        let function_data = parse_function_data(&syn::parse_str(TEST_CODE).unwrap());
        let python_function_data = analyze_function_data(&function_data);
        assert_eq!(
            python_function_data.args,
            vec![
                ("a".to_string(), "int".to_string()),
                ("b".to_string(), "int".to_string())
            ]
        );
    }

    #[test]
    fn test_analyze_function_return_type() {
        let function_data = parse_function_data(&syn::parse_str(TEST_CODE).unwrap());
        let python_function_data = analyze_function_data(&function_data);
        assert_eq!(python_function_data.return_type, "list[int]");
    }

    #[test]
    fn test_analyze_function_doc() {
        let function_data = parse_function_data(&syn::parse_str(TEST_CODE).unwrap());
        let python_function_data = analyze_function_data(&function_data);
        assert_eq!(python_function_data.doc, " testcode add function second line");
    }
}