const ERROR: &'static str = "The L macro only accepts one literal string as an argument";

#[proc_macro]
#[allow(non_snake_case)]
pub fn L(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn::Expr);
    let input = match input {
        syn::Expr::Lit(x) => x,
        _ => panic!(ERROR),
    };
    let input = match input.lit {
        syn::Lit::Str(x) => x,
        _ => panic!(ERROR),
    };
    let input = input.value();
    let mut output = "[".to_string();
    for i in encode_unicode::StrExt::utf16chars(input.as_str()) {
        output.push_str(i.to_string().as_str());
        output.push(',');
    }
    output.push(']');
    output.parse().unwrap()
}
