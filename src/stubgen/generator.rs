use tempfile::*;
use std::path::Path;
use std::error::Error;
use crate::stubgen::analyzer::*;
use std::fs::File;
use std::io::Write;


pub fn generate_stub(function_data: &PythonFunctionData, output_path: &Path) -> Result<(), Box<dyn Error>>{
    let mut file = File::create(output_path)?;
    writeln!(file, "# {} ", function_data.doc)?;
    writeln!(file, "def {}({}) -> {}", function_data.name, function_data.args.iter().map(|(name, ty)| format!("{}: {}", name, ty)).collect::<Vec<String>>().join(", "), function_data.return_type)?;
    Ok(())
}



#[cfg(test)]
mod tests{
    use super::*;
    use crate::stubgen::parser::*;

    const TEST_CODE: &str = 
    "
    /// testcode add function
    /// second line
    #[pyfunction]
    #[testattribute]
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    ";
    #[test]
    fn test_generate_stub(){
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_file = temp_dir.path().join("test.pyi");
        let function_data = parse_function_data(&syn::parse_str(TEST_CODE).unwrap());
        let python_function_data = analyze_function_data(&function_data);

        let stub = generate_stub(&python_function_data, &temp_file);

        assert!(temp_file.exists());

        let content = std::fs::read_to_string(temp_file).unwrap();
        println!("{}", content);
        assert_eq!(content, 
            "#  testcode add function second line\n
            def add(a: int, b: int) -> int");
    }
}