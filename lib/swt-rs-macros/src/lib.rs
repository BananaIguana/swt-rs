mod java_signature;
mod java_type;
mod macro_data;

use crate::macro_data::MacroData;

/// Macro function to bridge a Rust struct member function to an SWT Java class method function.
#[proc_macro]
pub fn swt_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{
    let macro_data = syn::parse_macro_input!(input as MacroData);

    let function_string = macro_data.as_member_fn();

    function_string.parse().unwrap()
}

/// Macro function to bridge a Rust struct member to an SWT Java field method.
#[proc_macro]
pub fn swt_field(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{
    let macro_data = syn::parse_macro_input!(input as MacroData);

    let function_string = macro_data.as_field_fn();

    function_string.parse().unwrap()
}
