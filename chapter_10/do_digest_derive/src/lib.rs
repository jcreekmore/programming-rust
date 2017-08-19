extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(DoDigest)]
pub fn do_digest(input: TokenStream) -> TokenStream {
    let s = input.to_string();

    let ast = syn::parse_derive_input(&s).unwrap();

    let gen = impl_do_digest(&ast);

    gen.parse().unwrap()
}

fn impl_do_digest(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;

    if let syn::Body::Struct(ref body) = ast.body {
        let field = match *body {
            syn::VariantData::Struct(ref fs) => {
                fs.iter().map(|f| f.ident.as_ref().unwrap().clone()).collect()
            },
            syn::VariantData::Tuple(ref fs) => {
                fs.iter().enumerate().map(|(i, _)| i.into()).collect()
            },
            _ => vec![],
        };

        quote! {
            impl<D> DoDigest<D> for #name
                where D: Digest
            {
                fn update_digest(&self, digest: &mut D) {
                    #( self.#field.update_digest(digest); )*
                }
            }
        }
    } else {
        panic!("#[derive(DoDigest) is only defined for structs");
    }
}
