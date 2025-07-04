use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse2, GenericParam, Generics, Ident, ItemStruct, Type};

pub fn define_struct(ident: &Ident, generics: &Generics) -> syn::Result<ItemStruct> {
    if generics.params.is_empty() {
        parse2(quote! {
            pub struct #ident;
        })
    } else {
        let mut generic_params = generics.params.clone();
        let mut phantom_params: Punctuated<Type, Comma> = Default::default();

        for param in generic_params.iter_mut() {
            match param {
                GenericParam::Type(type_param) => {
                    type_param.colon_token = None;
                    type_param.bounds.clear();

                    let type_ident = &type_param.ident;
                    phantom_params.push(parse2(quote!( #type_ident ))?);
                }
                GenericParam::Lifetime(life_param) => {
                    life_param.colon_token = None;
                    life_param.bounds.clear();

                    let lifetime = &life_param.lifetime;
                    phantom_params.push(parse2(quote!( & #lifetime () ))?);
                }
                _ => {}
            }
        }

        parse2(quote! {
            pub struct #ident < #generic_params > (
                pub ::core::marker::PhantomData<( #phantom_params )>
            );
        })
    }
}
