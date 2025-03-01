use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::delegate_components::impl_delegate_components;
use crate::parse::DelegateComponents;

pub fn delegate_components(body: TokenStream) -> syn::Result<TokenStream> {
    let ast: DelegateComponents = syn::parse2(body)?;

    let impl_items = impl_delegate_components(
        &ast.target_type,
        &ast.target_generics,
        &ast.delegate_entries,
    )?;

    let mut output = TokenStream::new();

    for impl_item in impl_items {
        output.extend(impl_item.to_token_stream());
    }

    Ok(output)
}
