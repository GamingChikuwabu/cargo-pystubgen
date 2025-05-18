// src/stubgen/type_map.rs

use syn::{Type, PathArguments, GenericArgument};

/// Rustの型をPython stub(.pyi)の型に変換する（PyO3のルールに基づく）
pub fn map_type(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => {
            let ident = type_path.path.segments.last().unwrap().ident.to_string();

            match ident.as_str() {
                // 数値型
                "i8" | "i16" | "i32" | "i64" | "isize" |
                "u8" | "u16" | "u32" | "u64" | "usize" => "int".to_string(),

                // 浮動小数点
                "f32" | "f64" => "float".to_string(),

                // 論理値
                "bool" => "bool".to_string(),

                // 文字列
                "String" => "str".to_string(),
                "PyString" => "str".to_string(),
                "Py<PyString>" => "str".to_string(),

                // Pythonオブジェクトそのまま
                "PyAny" => "Any".to_string(),
                "PyObject" => "Any".to_string(),
                "PyResult" => {
                    // PyResult<T> の T を再帰的に処理
                    extract_generic_type(type_path, 0).map_or("Any".to_string(), |inner| map_type(inner))
                }

                // オプション型
                "Option" => {
                    extract_generic_type(type_path, 0).map_or("Any | None".to_string(), |inner| {
                        format!("{} | None", map_type(inner))
                    })
                }

                // ベクタ型
                "Vec" => {
                    extract_generic_type(type_path, 0).map_or("list[Any]".to_string(), |inner| {
                        format!("list[{}]", map_type(inner))
                    })
                }

                // ハッシュマップ
                "HashMap" => {
                    let key_type = extract_generic_type(type_path, 0);
                    let val_type = extract_generic_type(type_path, 1);
                    let key = key_type.map_or("Any".to_string(), |k| map_type(k));
                    let val = val_type.map_or("Any".to_string(), |v| map_type(v));
                    format!("dict[{}, {}]", key, val)
                }

                // その他：とりあえず型名をそのまま返す
                _ => ident,
            }
        }

        // 参照型（&T）
        Type::Reference(r) => map_type(&r.elem),

        // タプル型など未対応の型は Any 扱い
        _ => "Any".to_string(),
    }
}

/// Generic typeを抽出するヘルパー関数（Vec<T>などのTを取得）
fn extract_generic_type(type_path: &syn::TypePath, index: usize) -> Option<&Type> {
    type_path.path.segments.last().and_then(|seg| {
        if let PathArguments::AngleBracketed(ref generics) = seg.arguments {
            generics.args.iter().filter_map(|arg| {
                if let GenericArgument::Type(ty) = arg {
                    Some(ty)
                } else {
                    None
                }
            }).nth(index)
        } else {
            None
        }
    })
}


#[cfg(test)]
mod tests{
    use super::*;
    

    #[test]
    fn test_map_type(){
        let ty = syn::parse_str("i32").unwrap();
        let result = map_type(&ty);
        assert_eq!(result, "int");
    }


    #[test]
    fn test_map_type_with_generic(){
        let ty = syn::parse_str("Vec<i32>").unwrap();
        let result = map_type(&ty);
        assert_eq!(result, "list[int]");
    }
    

    #[test]
    fn test_map_type_with_option(){
        let ty = syn::parse_str("Option<i32>").unwrap();
        let result = map_type(&ty);
        assert_eq!(result, "int | None");
    }
    

    #[test]
    fn test_map_type_with_tuple(){
        let ty = syn::parse_str("(i32, i32)").unwrap();
        let result = map_type(&ty);
        assert_eq!(result, "tuple[int, int]");
    }
    
}