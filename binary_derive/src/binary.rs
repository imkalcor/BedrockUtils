use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{
    parse2, parse_quote, Data, DeriveInput, Error, Fields, GenericParam, Generics, Lifetime,
    LifetimeParam, LitInt,
};
use syn::{Attribute, LitStr, Result, Variant};

/// Derives the Binary trait on Structs and Enums for serialization and deserialization purposes.
pub fn binary_derive(item: TokenStream) -> Result<TokenStream> {
    let mut input = parse2::<DeriveInput>(item.into()).unwrap();
    let name = input.ident;

    if input.generics.lifetimes().count() > 1 {
        return Err(Error::new(
            input.generics.params.span(),
            "type deriving `Binary` must have no more than one lifetime",
        ));
    }

    // Use the lifetime specified in the type definition or just use `'a` if not
    // present.
    let lifetime = input
        .generics
        .lifetimes()
        .next()
        .map(|l| l.lifetime.clone())
        .unwrap_or_else(|| parse_quote!('a));

    match input.data {
        Data::Struct(struct_) => {
            let serialize = match &struct_.fields {
                Fields::Named(fields) => fields
                    .named
                    .iter()
                    .filter_map(|f| {
                        let name = &f.ident.as_ref().unwrap();

                        for attr in &f.attrs {
                            if attr.path().is_ident("skip") {
                                return None;
                            }
                        }

                        Some(quote! {
                            self.#name.serialize(buf);
                        })
                    })
                    .collect(),
                Fields::Unnamed(fields) => (0..fields.unnamed.len())
                    .map(|i| {
                        let lit = LitInt::new(&i.to_string(), Span::call_site());

                        quote! {
                            self.#lit.serialize(buf);
                        }
                    })
                    .collect(),
                Fields::Unit => TokenStream::new(),
            };

            let deserialize = match struct_.fields {
                Fields::Named(fields) => {
                    let init = fields.named.iter().map(|f| {
                        let name = f.ident.as_ref().unwrap();

                        for attr in &f.attrs {
                            if attr.path().is_ident("skip") {
                                return quote! {
                                    #name: Default::default(),
                                };
                            }
                        }

                        quote! {
                            #name: Binary::deserialize(buf)?,
                        }
                    });

                    quote! {
                        Self {
                            #(#init)*
                        }
                    }
                }
                Fields::Unnamed(fields) => {
                    let init = (0..fields.unnamed.len())
                        .map(|_| {
                            quote! {
                                Binary::deserialize(buf)?,
                            }
                        })
                        .collect::<TokenStream>();

                    quote! {
                        Self(#init)
                    }
                }
                Fields::Unit => quote!(Self),
            };

            add_trait_bounds(
                &mut input.generics,
                quote!(::valence_protocol::__private::Decode<#lifetime>),
            );

            let (impl_generics, ty_generics, where_clause) =
                decode_split_for_impl(input.generics, lifetime.clone());

            Ok(quote! {
                #[allow(unused_imports)]
                impl #impl_generics ::binary::Binary<#lifetime> for #name #ty_generics
                #where_clause
                {
                    fn serialize(&self, buf: &mut impl Write) {
                        use bytes::BytesMut;
                        use ::binary::Binary;

                        #serialize
                    }

                    fn deserialize(buf: &mut std::io::Cursor<&'a [u8]>) -> std::io::Result<Self> {
                        use bytes::BytesMut;
                        use ::binary::Binary;

                        Ok(#deserialize)
                    }
                }
            })
        }
        Data::Enum(enum_) => {
            let mut datatype = None;

            for attr in &input.attrs {
                if attr.path().is_ident("data") {
                    let _ = attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("datatype") {
                            let value = meta.value()?;
                            let v: LitStr = value.parse()?;

                            datatype = Some(v.value());
                        }

                        Ok(())
                    });
                }
            }

            let datatype = match datatype {
                Some(val) => val,
                None => String::from("VarI32"),
            };

            let datatypes = [
                "I8", "U8", "I16", "U16", "I32", "U32", "I16BE", "U16BE", "I32BE", "U32BE",
                "VarI32", "VarU32",
            ];

            if !datatypes.contains(&datatype.as_str()) {
                return Ok(quote! {
                    compile_error!("Datatypes can only be of type I8, U8, I16, U16, I32, U32, I16BE, U16BE, I32BE, U32BE, VarI32, VarU32");
                });
            }

            let variants = pair_variants_with_discriminants(enum_.variants)?;

            let serialize = variants
                .iter()
                .map(|(disc, variant)| {
                    let variant_name = &variant.ident;

                    let encode_disc = quote! {
                        match #datatype {
                            "I8" => I8::new(#disc as i8).serialize(buf),
                            "U8" => U8::new(#disc as u8).serialize(buf),
                            "I16" => I16::<LE>::new(#disc as i16).serialize(buf),
                            "U16" => U16::<LE>::new(#disc as u16).serialize(buf),
                            "I32" => I32::<LE>::new(#disc as i32).serialize(buf),
                            "U32" => U32::<LE>::new(#disc as u32).serialize(buf),
                            "I16BE" => I16::<BE>::new(#disc as i16).serialize(buf),
                            "U16BE" => U16::<BE>::new(#disc as u16).serialize(buf),
                            "I32BE" => I32::<BE>::new(#disc as i32).serialize(buf),
                            "U32BE" => U32::<BE>::new(#disc as u32).serialize(buf),
                            "VarI32" => VarI32::new(#disc as i32).serialize(buf),
                            "VarU32" => VarU32::new(#disc as u32).serialize(buf),
                            _ => panic!("Unable to find the datatype by the name {:?}", #datatype)
                        }
                    };

                    match &variant.fields {
                        Fields::Named(fields) => {
                            let field_names = fields
                                .named
                                .iter()
                                .map(|f| f.ident.as_ref().unwrap())
                                .collect::<Vec<_>>();

                            let encode_fields = field_names
                                .iter()
                                .map(|name| {
                                    quote! {
                                        #name.serialize(buf);
                                    }
                                })
                                .collect::<TokenStream>();

                            quote! {
                                Self::#variant_name { #(#field_names,)* } => {
                                    #encode_disc
                                    #encode_fields
                                }
                            }
                        }
                        Fields::Unnamed(fields) => {
                            let field_names = (0..fields.unnamed.len())
                                .map(|i| Ident::new(&format!("_{i}"), Span::call_site()))
                                .collect::<Vec<_>>();

                            let encode_fields = field_names
                                .iter()
                                .map(|name| {
                                    quote! {
                                        #name.serialize(buf);
                                    }
                                })
                                .collect::<TokenStream>();

                            quote! {
                                Self::#variant_name(#(#field_names,)*) => {
                                    #encode_disc
                                    #encode_fields
                                }
                            }
                        }
                        Fields::Unit => {
                            quote! {
                                 Self::#variant_name => #encode_disc,
                            }
                        }
                    }
                })
                .collect::<TokenStream>();

            let deserialize = variants
                .iter()
                .map(|(disc, variant)| {
                    let variant_name = &variant.ident;

                    match &variant.fields {
                        Fields::Named(fields) => {
                            let fields = fields
                                .named
                                .iter()
                                .map(|f| {
                                    let field = f.ident.as_ref().unwrap();

                                    quote! {
                                        #field: Binary::deserialize(buf)?,
                                    }
                                })
                                .collect::<TokenStream>();

                            quote! {
                                #disc => Ok(Self::#variant_name { #fields }),
                            }
                        }
                        Fields::Unnamed(fields) => {
                            let fields = (0..fields.unnamed.len())
                                .map(|_| {
                                    quote! {
                                        Binary::deserialize(buf)?,
                                    }
                                })
                                .collect::<TokenStream>();

                            quote! {
                                #disc => Ok(Self::#variant_name(#fields)),
                            }
                        }
                        Fields::Unit => quote!(#disc => Ok(Self::#variant_name),),
                    }
                })
                .collect::<TokenStream>();

            add_trait_bounds(
                &mut input.generics,
                quote!(::valence_protocol::__private::Decode<#lifetime>),
            );

            let (impl_generics, ty_generics, where_clause) =
                decode_split_for_impl(input.generics, lifetime.clone());

            Ok(quote! {
                 #[allow(unused_imports)]
                impl #impl_generics ::binary::Binary<#lifetime> for #name #ty_generics
                #where_clause
                {
                    fn serialize(&self, buf: &mut impl Write) {
                        use bytes::BytesMut;
                        use ::binary::Binary;
                        use ::binary::datatypes::{I8, U8, I16, U16, I32, U32, VarI32, VarU32};
                        use byteorder::{BE, LE};

                        match self {
                            #serialize
                            _ => unreachable!(),
                        }
                    }

                    fn deserialize(buf: &mut std::io::Cursor<&'a [u8]>) -> std::io::Result<Self> {
                        use bytes::BytesMut;
                        use ::binary::Binary;
                        use ::binary::datatypes::{I8, U8, I16, U16, I32, U32, VarI32, VarU32};
                        use byteorder::{BE, LE};

                        let disc = match #datatype {
                            "I8" => I8::deserialize(buf)?.0 as usize,
                            "U8" => U8::deserialize(buf)?.0 as usize,
                            "U16" => U16::<LE>::deserialize(buf)?.0 as usize,
                            "I16" => I16::<LE>::deserialize(buf)?.0 as usize,
                            "I32" => I32::<LE>::deserialize(buf)?.0 as usize,
                            "U32" => U32::<LE>::deserialize(buf)?.0 as usize,
                            "U16BE" => U16::<BE>::deserialize(buf)?.0 as usize,
                            "I16BE" => I16::<BE>::deserialize(buf)?.0 as usize,
                            "I32BE" => I32::<BE>::deserialize(buf)?.0 as usize,
                            "U32BE" => U32::<BE>::deserialize(buf)?.0 as usize,
                            "VarI32" => VarI32::deserialize(buf)?.0 as usize,
                            "VarU32" => VarU32::deserialize(buf)?.0 as usize,
                            _ => panic!("Unable to find the datatype by the name {:?}", #datatype)
                        };

                        match disc {
                            #deserialize
                            n => return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Unexpected enum discriminant {:?}", n)))
                        }
                    }
                }
            })
        }
        Data::Union(union) => Err(Error::new(
            union.union_token.span,
            "Cannot derive `Binary` on unions",
        )),
    }
}

/// Pairs the variants from the Iterator passed into a Vector of a tuple of the discriminant
/// and the variant.
fn pair_variants_with_discriminants(
    variants: impl IntoIterator<Item = Variant>,
) -> Result<Vec<(usize, Variant)>> {
    let mut discriminant = 0;
    variants
        .into_iter()
        .map(|v| {
            if let Some(i) = parse_tag_attr(&v.attrs)? {
                discriminant = i;
            }

            let pair = (discriminant, v);
            discriminant += 1;

            Ok(pair)
        })
        .collect::<Result<_>>()
}

/// Parses the tag attribute and returns it if exists.
fn parse_tag_attr(attrs: &[Attribute]) -> Result<Option<usize>> {
    for attr in attrs {
        if attr.path().is_ident("variant") {
            let mut res = 0;

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("tag") {
                    res = meta.value()?.parse::<LitInt>()?.base10_parse::<usize>()?;
                    Ok(())
                } else {
                    Err(meta.error("unrecognized argument"))
                }
            })?;

            return Ok(Some(res));
        }
    }

    Ok(None)
}

/// Adding our lifetime to the generics before calling `.split_for_impl()` would
/// also add it to the resulting ty_generics, which we don't want. So I'm doing
/// this hack.
fn decode_split_for_impl(
    mut generics: Generics,
    lifetime: Lifetime,
) -> (TokenStream, TokenStream, TokenStream) {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut impl_generics = impl_generics.to_token_stream();
    let ty_generics = ty_generics.to_token_stream();
    let where_clause = where_clause.to_token_stream();

    if generics.lifetimes().next().is_none() {
        generics
            .params
            .push(GenericParam::Lifetime(LifetimeParam::new(lifetime)));

        impl_generics = generics.split_for_impl().0.to_token_stream();
    }

    (impl_generics, ty_generics, where_clause)
}

fn add_trait_bounds(generics: &mut Generics, trait_: TokenStream) {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(parse_quote!(#trait_))
        }
    }
}
