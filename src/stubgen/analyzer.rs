use crate::stubgen::parser::*;
use crate::stubgen::typemap::*;

pub struct PythonFunctionData{
    pub name: String,
    pub args: Vec<(String, String)>,
    pub return_type: String,
    pub doc: String,
}

pub fn analyze_function_data(function_data: &RustFunctionData) -> PythonFunctionData{
    PythonFunctionData{
        name: analyze_function_name(function_data),
        args: analyze_function_args(function_data),
        return_type: analyze_function_return_type(function_data),
        doc: analyze_function_doc(function_data),
    }
}

pub fn analyze_function_name(function_data: &RustFunctionData) -> String{
    function_data.name.clone()
}

pub fn analyze_function_args(function_data: &RustFunctionData) -> Vec<(String, String)>{
    function_data.args.iter()
        .map(|(name, ty)|{
            let rust_type = syn::parse_str::<syn::Type>(ty).unwrap();
            let python_type = map_type(&rust_type);
            (name.clone(), python_type)
        }).collect()
}

pub fn analyze_function_return_type(function_data: &RustFunctionData) -> String{
    let rust_type = syn::parse_str::<syn::Type>(&function_data.return_type).unwrap();
    map_type(&rust_type)
}

pub fn analyze_function_doc(function_data: &RustFunctionData) -> String{
    function_data.doc.clone()
}


#[cfg(test)]
mod tests{
    use super::*;

    const TEST_CODE: &str = 
    "
    /// testcode add function
    /// second line
    #[pyfunction]
    #[testattribute]
    fn add(a: i32, b: i32) -> Vec<i32> {
        vec![a + b]
    }
    ";
    #[test]
    fn test_analyze_function_name(){
        let function_data = parse_function_data(&syn::parse_str(TEST_CODE).unwrap());
        let python_function_data = analyze_function_data(&function_data);
        assert_eq!(python_function_data.name, "add");
    }

    #[test]
    fn test_analyze_function_args(){
        let function_data = parse_function_data(&syn::parse_str(TEST_CODE).unwrap());
        let python_function_data = analyze_function_data(&function_data);
        assert_eq!(python_function_data.args, vec![("a".to_string(), "int".to_string()), ("b".to_string(), "int".to_string())]);
    }

    #[test]
    fn test_analyze_function_return_type(){
        let function_data = parse_function_data(&syn::parse_str(TEST_CODE).unwrap());
        let python_function_data = analyze_function_data(&function_data);
        assert_eq!(python_function_data.return_type, "list[int]");
    }   

    #[test]
    fn test_analyze_function_doc(){
        let function_data = parse_function_data(&syn::parse_str(TEST_CODE).unwrap());
        let python_function_data = analyze_function_data(&function_data);
        assert_eq!(python_function_data.doc, " testcode add function second line");
    }
}