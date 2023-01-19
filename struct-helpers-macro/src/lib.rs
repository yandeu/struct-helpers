extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Literal, TokenTree};
use quote::{format_ident, quote, TokenStreamExt};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Helpers, attributes(helper))]
pub fn my_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!();
    };

    let mut body = quote! {};

    for f in fields.iter() {
        let field_name = f.ident.as_ref().unwrap();

        let mut is_option = false;
        let ty = &f.ty;

        if let syn::Type::Path(ref p) = ty {
            let path_segment = p.path.segments.iter().next().unwrap();
            let field_type = path_segment.ident.to_string();
            is_option = field_type == "Option";
        }

        for attr in f.attrs.iter() {
            #[derive(Debug, Clone)]
            struct Fnn {
                ident: Ident,
                params: Vec<Literal>,
            }

            // filter all attributes that are not "helper"
            let ident = attr.path.segments.first().unwrap().ident.to_string();
            if ident != "helper" {
                continue;
            }

            for f in attr.tokens.clone().into_iter() {
                let mut fnns: Vec<Fnn> = vec![];

                if let TokenTree::Group(group) = f {
                    for f1 in group.stream().into_iter() {
                        match f1 {
                            TokenTree::Group(ref group) => {
                                for f2 in group.stream().into_iter() {
                                    if let TokenTree::Literal(literal) = f2 {
                                        if let Some(e) = fnns.last_mut() {
                                            e.params.push(literal);
                                        }
                                    }
                                }
                            }
                            TokenTree::Ident(ident) => {
                                let f = Fnn {
                                    ident,
                                    params: vec![],
                                };
                                fnns.push(f);
                            }
                            _ => {}
                        }
                    }
                }

                for f in fnns.into_iter() {
                    let mut params_q = quote! {};

                    for lit in f.params.iter() {
                        params_q.append_all(quote! {,});
                        params_q.append_all(quote! {#lit.clone()})
                    }

                    let ident_string = f.ident.clone().to_string();
                    let _field_name_string = field_name.clone().to_string();

                    let function_name = format_ident!("{}", &ident_string);
                    let function_name_optional = format_ident!("{}_optional", &ident_string);

                    if !is_option {
                        body.append_all({
                            quote! {
                                #[allow(clippy::unnecessary_mut_passed)]
                                if let Err(e) = #function_name(&mut self.#field_name #params_q) {
                                    error = Some(e.to_string());
                                    // error = ["Error in fn", #ident_string, "for field",#field_name_string ].join(" ");
                                };
                            }
                        });
                    } else {
                        body.append_all({
                            quote! {
                                #[allow(clippy::unnecessary_mut_passed)]
                                if let Err(e) = #function_name_optional(&mut self.#field_name #params_q) {
                                    error = Some(e.to_string());
                                    // error = ["Error in fn", #ident_string, "for field",#field_name_string ].join(" ");
                                };
                            }
                        });
                    }
                }
            }
        }
    }

    let output = quote! {
        impl Helpers for #name {
            fn run_helpers(&mut self) -> Result<(), String> {
                let mut error:Option<String> = None;
                #body
                match error {
                    Some(e)=> {
                        println!("ERROR: {}", e);
                        Err(e)
                    },
                    None => {
                        Ok(())
                    }
                }
            }
        }
    };

    TokenStream::from(output)
}
