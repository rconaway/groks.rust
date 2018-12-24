extern crate proc_macro;

use crate::proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn grok(attr: TokenStream, item: TokenStream) -> TokenStream {
    use std::str::FromStr;

    let name = attr.to_string();
    let code = item.to_string();

    let code = String::from("#[test]\n") + &code.replace("test", &name);
    TokenStream::from_str(&code).unwrap()

}


