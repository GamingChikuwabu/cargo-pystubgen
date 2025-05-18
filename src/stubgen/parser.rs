use quote::ToTokens;
use syn::Meta;

#[derive(Debug,Default,PartialEq, Eq)]
pub struct RustFunctionData{
    pub name: String,
    pub args: Vec<(String, String)>,
    pub return_type: String,
    pub attributes: Vec<String>,
    pub doc: String,
}

pub fn parse_function_data(item: &syn::ItemFn) -> RustFunctionData{
    RustFunctionData{
        name: parse_function_name(item),
        args: parse_function_args(item),
        return_type: parse_function_return_type(item),
        attributes: parse_function_attributes(item),
        doc: parse_function_doc(item),
    }
}

pub fn parse_function_name(item: &syn::ItemFn) -> String{
    item.sig.ident.to_string()
}

pub fn parse_function_args(item: &syn::ItemFn) -> Vec<(String, String)>{
    item.sig.inputs.iter().map(|arg|match arg{
        syn::FnArg::Typed(arg) => (arg.pat.to_token_stream().to_string(), arg.ty.to_token_stream().to_string()),
        _ => ("self".to_string(), "self".to_string()),
    })
    .collect()
}

pub fn parse_function_return_type(item: &syn::ItemFn) -> String{
    match &item.sig.output {
        syn::ReturnType::Default => "()".to_string(),
        syn::ReturnType::Type(_, ty) => ty.to_token_stream().to_string(),
    }
}

pub fn parse_function_attributes(item: &syn::ItemFn) -> Vec<String> {
    item.attrs.iter()
        .filter_map(|attr| match &attr.meta {
            Meta::Path(path) if !attr.path().is_ident("doc") => {
                Some(path.segments.last().unwrap().ident.to_string())
            }
            _ => None,
        })
        .collect()
}


pub fn parse_function_doc(item: &syn::ItemFn) -> String{
    let mut doc = String::new();
    item.attrs.iter().for_each(|attr|{
        match &attr.meta {
            Meta::NameValue(name_value) =>match &name_value.value {
                syn::Expr::Lit(lit_str) => {
                    let doc_str = match &lit_str.lit {
                        syn::Lit::Str(lit_str) => lit_str.value(),
                        _ => "".to_string(),
                    };
                    doc.push_str(&doc_str);
                }
                _ => {}
            }
            Meta::Path(_) => {},
            Meta::List(_) => {},
        }
    });
    doc
}


#[cfg(test)]
mod tests {
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
    fn test_parse_function_return_type(){
        use super::*;
        let item: syn::ItemFn = syn::parse_str(TEST_CODE).unwrap();
        let function_data = parse_function_data(&item);

        assert_eq!(function_data.return_type, "i32");
    }

    #[test]
    fn test_parse_function_name(){
        use super::*;
        let item: syn::ItemFn = syn::parse_str(TEST_CODE).unwrap();
        let function_data = parse_function_data(&item);

        assert_eq!(function_data.name, "add");
    }

    #[test]
    fn test_parse_function_args(){
        use super::*;
        let item: syn::ItemFn = syn::parse_str(TEST_CODE).unwrap();
        let function_data = parse_function_data(&item);

        assert_eq!(function_data.args, vec![("a".to_string(), "i32".to_string()), ("b".to_string(), "i32".to_string())]);
    }

    #[test]
    fn test_parse_function_attributes(){
        use super::*;
        let item: syn::ItemFn = syn::parse_str(TEST_CODE).unwrap();
        let function_data = parse_function_data(&item);

        assert_eq!(function_data.attributes, vec!["pyfunction".to_string(), "testattribute".to_string()]);
    }

    #[test]
    fn test_parse_function_comments(){
        use super::*;
        let item: syn::ItemFn = syn::parse_str(TEST_CODE).unwrap();
        let function_data = parse_function_data(&item);

        assert_eq!(function_data.doc, " testcode add function second line");
    }

   
}
