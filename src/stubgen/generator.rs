//! スタブファイル生成モジュール
//! 
//! このモジュールは、解析されたRustコードの情報からPythonの型ヒント付きスタブファイルを生成します。

use std::path::Path;
use std::error::Error;
use crate::stubgen::analyzer::*;
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;

/// Pythonスタブファイルを生成する
/// 
/// # Arguments
/// 
/// * `python_src_data` - 解析されたPython関数データ
/// * `output_dir` - 出力ディレクトリ
/// * `module_name` - モジュール名
/// 
/// # Returns
/// 
/// * `Result<(), Box<dyn Error>>` - 処理結果
pub fn generate_stub(
    python_src_data: &PythonSrcData,
    output_dir: &Path,
    module_name: &str,
) -> Result<(), Box<dyn Error>> {
    let output_path = output_dir.join(format!("{}.pyi", module_name));
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&output_path)
        .map_err(|e| format!("File create failed: {}", e))?;

    println!("Generating stub file: {}", output_path.display());

    for function_data in &python_src_data.functions {
        if function_data.name == module_name {
            continue;
        }
        generate_function_stub(&mut file, function_data)?;
    }

    file.flush()?;

    Ok(())
}

/// 個々の関数のスタブを生成する
/// 
/// # Arguments
/// 
/// * `file` - 出力ファイル
/// * `function_data` - 関数データ
/// 
/// # Returns
/// 
/// * `Result<(), Box<dyn Error>>` - 処理結果
fn generate_function_stub(
    file: &mut File,
    function_data: &PythonFunctionData,
) -> Result<(), Box<dyn Error>> {

    if !function_data.doc.is_empty() {
        writeln!(file, "# {}", function_data.doc)?;
    }

    let args_str = if function_data.args.is_empty() {
        "".to_string()
    } else {
        function_data
            .args
            .iter()
            .map(|(name, ty)| format!("{}: {}", name, ty))
            .collect::<Vec<String>>()
            .join(", ")
    };

    
    writeln!(
        file,
        "def {}({}) -> {}:",
        function_data.name, args_str, function_data.return_type
    )?;
    writeln!(file, "    ...")?;
    writeln!(file)?;

    file.flush()?;
    Ok(())
}
