//! Pythonスタブファイル生成ツール
//! 
//! このモジュールは、RustのソースコードからPythonの型ヒント付きスタブファイル（.pyi）を生成します。
//! maturinを使用したPythonプロジェクトの型情報を自動的に生成します。

pub mod parser;    // Rustソースコードのパース
pub mod analyzer;  // パースされたRustコードの解析
pub mod typemap;   // Rust型からPython型への変換
pub mod generator; // スタブファイルの生成
pub mod locator;   // Pythonプロジェクトの検出

/// Pythonスタブファイルを生成する
/// 
/// # Arguments
/// 
/// * `project_dir` - プロジェクトのルートディレクトリ（オプション）
/// * `output_dir` - 出力ディレクトリ（オプション）
/// * `debug` - デバッグモードの有効/無効
pub fn generate_stubs(debug: bool) {
    // プロジェクト情報の取得
    let project_infos = locator::locate_python_project().unwrap();

    for project_info in project_infos {
        println!("Processing project: {}", project_info.project_name);
        println!("Output directory: {}", project_info.output_dir.display());
        println!("Module name: {}", project_info.module_name);

        // 既存のスタブファイルを削除
        let stub_file = project_info.output_dir.join(format!("{}.pyi", project_info.module_name));
        if stub_file.exists() {
            std::fs::remove_file(&stub_file)
                .unwrap_or_else(|e| println!("Failed to remove existing stub file: {}", e));
        }

        // 各Rustソースファイルを処理
        for rust_src_file in project_info.rust_src_file {
            if debug {
                println!("Processing file: {}", rust_src_file.display());
            }

            // Rustソースコードの解析
            let parsed_rust_data = parser::parse_rust_src_file(&rust_src_file);
            
            // Python関数データへの変換
            let python_function_data = analyzer::analyze_rust_src_data(&parsed_rust_data);
            
            // スタブファイルの生成
            generator::generate_stub(
                &python_function_data,
                &project_info.output_dir,
                &project_info.module_name,
            ).unwrap();
        }
    }
}