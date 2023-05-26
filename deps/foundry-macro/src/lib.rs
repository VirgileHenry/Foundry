

#[proc_macro_derive(AsAny)]
pub fn as_any_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;
    quote::quote! {
        impl AsAny for #name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    }.into()
}