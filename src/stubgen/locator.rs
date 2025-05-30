// pythonのプロジェクトを解析して、モジュールの位置を探す

use std::env;
use std::error::Error;
use std::path::PathBuf;
use toml_edit::DocumentMut;



#[derive(Debug)]
pub struct ProjectInfo{
    pub project_name:String,
    pub module_name:String,
    pub output_dir:PathBuf,
    pub rust_src_file:Vec<PathBuf>,
}

pub fn locate_python_project()->Result<Vec<ProjectInfo>, Box<dyn Error>>{
    let current_dir = env::current_dir().expect("現在のディレクトリを取得できませんでした");
    // pyproject.tomlを探し解析してワークスペースか、単一プロジェクトかを判断する
    let mut project_infos = vec![];
    get_maturin_project_info(current_dir.to_path_buf(),&mut project_infos)?;
    Ok(project_infos)
}

/// プロジェクトの情報を集める
/// 
/// # Arguments
/// 
/// * `project_root` - プロジェクトのルートディレクトリ
/// * `project_infos` - プロジェクトの情報を格納するベクター
/// 
/// # Returns
/// 
/// 
fn get_maturin_project_info(project_root:PathBuf,project_infos:&mut Vec<ProjectInfo>)->Result<(), Box<dyn Error>>{
    if project_root.join("pyproject.toml").exists(){
        // ワークスペース
        let pyproject_toml = std::fs::read_to_string(project_root.join("pyproject.toml")).unwrap();
        let doc = pyproject_toml.parse::<DocumentMut>().expect("TOMLパース失敗");

        if let Some(uv_workspace) = doc.get("tool")
                                   .and_then(|tool| tool.get("uv"))
                                   .and_then(|uv| uv.get("workspace")){
            let members = get_workspace_members_path(uv_workspace,project_root);
            for member in members {
                get_maturin_project_info(member, project_infos)?;
            }
        }
        else{
            if is_maturin_project(&doc){
                let cargo_toml = std::fs::read_to_string(project_root.join("Cargo.toml")).unwrap();
                let cargo_doc = cargo_toml.parse::<DocumentMut>().unwrap();
                let mut rust_src_files = vec![];
                get_rust_src_files(project_root.join("src"),&mut rust_src_files)?;
                let output_dir = project_root.join("src").join(get_project_name(&cargo_doc));
                project_infos.push(ProjectInfo {
                    project_name: get_project_name(&cargo_doc),
                    module_name: get_module_name(&cargo_doc),
                    rust_src_file: rust_src_files,
                    output_dir: output_dir
                });
            }
        }
    }
    else{
        //pythonのプロジェクトではないのでエラー
        return Err(format!("{}はpyproject.tomlを発見できなかったため解析できませんでした", project_root.display()).into());
    }

    Ok(())
}

/// pyproject.tomlの[build-system]セクションのbuild-backendがmaturinであるかどうかを確認する
fn is_maturin_project(pyproject_toml:&toml_edit::DocumentMut)->bool{
    let build_system = pyproject_toml.get("build-system").unwrap();
    let build_backend = build_system.get("build-backend").unwrap();
    build_backend.as_str().unwrap() == "maturin"
}

/// pyproject.tomlの[tool.uv.workspace]セクションからワークスペースのメンバーを取得する
/// 
/// # Arguments
/// 
/// * `workspace_section` - pyproject.tomlの[tool.uv.workspace]セクション
/// * `workspace_root` - ワークスペースのルートディレクトリ
///
/// # Returns
/// 
/// ワークスペースのメンバーのパス
fn get_workspace_members_path(workspace_section:&toml_edit::Item,workspace_root:PathBuf)->Vec<PathBuf>{
    let members = workspace_section.get("members").unwrap();
    members.as_array().unwrap().iter()
    .map(|v|{
        //memberが/*で終わっているかどうかを確認する
        if v.as_str().unwrap().ends_with("/*"){
            // [foo/*]で終わっている場合は、ディレクトリを再帰的に探索する
            let path = v.as_str().unwrap().trim_end_matches("/*");
            let dir = workspace_root.join(path);
            dir.read_dir().unwrap().map(|entry| entry.unwrap().path()).collect()
        }
        else{
            // [foo/*]で終わっていない場合は、そのままパスを返す
            vec![workspace_root.join(v.as_str().unwrap())]
        }
    })
    .flatten().collect()
}


/// pyproject.tomlの[project]セクションからプロジェクト名を取得する
/// 
/// # Arguments
/// 
/// * `toml_doc` - pyproject.tomlのDocumentMut
/// 
/// # Returns
/// 
/// プロジェクト名
fn get_project_name(toml_doc:&toml_edit::DocumentMut)->String{
    let project_section = toml_doc.get("package").unwrap();
    let name = project_section.get("name").unwrap();
    name.as_str().unwrap().to_string()
}

/// Cargo.tomlの[lib]セクションからモジュール名を取得する
/// 
/// 
/// # Arguments
/// 
/// * `toml_doc` - Cargo.tomlのDocumentMut
/// 
/// # Returns
/// 
/// モジュール名
fn get_module_name(toml_doc:&toml_edit::DocumentMut)->String{
    let lib_section = toml_doc.get("lib").unwrap();
    let name = lib_section.get("name").unwrap();
    name.as_str().unwrap().to_string()
}

/// srcディレクトリ以下のRustソースファイルを再帰的に探索する
fn get_rust_src_files(src_root:PathBuf,rust_src_files:&mut Vec<PathBuf>)->Result<(), Box<dyn Error>>{
    let entries = std::fs::read_dir(src_root)?;
    for entry in entries{
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap() == "rs"{
            rust_src_files.push(path);
        }
        else if path.is_dir(){
            get_rust_src_files(path,rust_src_files)?;
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests{
    use super::*;
    use std::path::PathBuf;

    const TEST_PROJECT_ROOT: &str = "tests\\test-project";
    const TEST_PROJECT_ROOT_SINGLE: &str = "tests\\test-project\\single_project";

    #[test]
    fn test_is_maturin_project(){
        // テスト用のTOMLデータ
        let maturin_toml = r#"
        [build-system]
        build-backend = "maturin"
        "#;

        let non_maturin_toml = r#"
        [build-system]
        build-backend = "setuptools"
        "#;

        // maturinプロジェクトのテスト
        let toml_doc = maturin_toml.parse::<DocumentMut>().unwrap();
        assert!(is_maturin_project(&toml_doc), "maturinプロジェクトとして認識されるべき");

        // 非maturinプロジェクトのテスト
        let toml_doc = non_maturin_toml.parse::<DocumentMut>().unwrap();
        assert!(!is_maturin_project(&toml_doc), "maturinプロジェクトとして認識されるべきではない");
    }

    #[test]
    fn test_get_workspace_members_path(){
        let current_dir = env::current_dir().unwrap();
        let project_root = PathBuf::from(current_dir.join(TEST_PROJECT_ROOT));
        let pyproject_toml = std::fs::read_to_string(project_root.join("pyproject.toml")).unwrap();
        let toml_doc = pyproject_toml.parse::<DocumentMut>().unwrap();
        let workspace_section = toml_doc.get("tool").unwrap().get("uv").unwrap().get("workspace").unwrap();
        let members = get_workspace_members_path(workspace_section,project_root);
        assert_eq!(members.len(), 4);
        assert_eq!(members[0], PathBuf::from(current_dir.join(TEST_PROJECT_ROOT).join("libs").join("lib_a")));
        assert_eq!(members[1], PathBuf::from(current_dir.join(TEST_PROJECT_ROOT).join("libs").join("lib_b")));
        assert_eq!(members[2], PathBuf::from(current_dir.join(TEST_PROJECT_ROOT).join("libs").join("lib_c")));
        assert_eq!(members[3], PathBuf::from(current_dir.join(TEST_PROJECT_ROOT).join("single_project")));
    }

    #[test]
    fn test_get_project_info(){
        let current_dir = env::current_dir().unwrap();
        let project_root = PathBuf::from(current_dir.join(TEST_PROJECT_ROOT));
        let mut project_infos = vec![];
        get_maturin_project_info(project_root,&mut project_infos).unwrap();
        assert_eq!(project_infos.len(), 2);
    }

    #[test]
    fn test_get_rust_src_files(){
        let current_dir = env::current_dir().unwrap();
        let project_root = PathBuf::from(current_dir.join(TEST_PROJECT_ROOT_SINGLE).join("src"));
        let mut rust_src_files = vec![];
        get_rust_src_files(project_root,&mut rust_src_files).unwrap();
        assert_eq!(rust_src_files.len(), 4);
    }
}