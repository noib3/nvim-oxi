use core::cmp::Ordering;

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::*;

struct OptsField {
    name: Ident,
    ty: Type,
    doc_comment: Option<String>,
    mask_idx: usize,
}

impl From<&Field> for OptsField {
    fn from(field: &Field) -> Self {
        Self {
            name: field.ident.as_ref().unwrap().clone(),
            ty: field.ty.clone(),
            doc_comment: parse_doc_comment(field),
            mask_idx: 0,
        }
    }
}

/// TODO: docs
pub fn derive_opts_builder(attr: TokenStream) -> TokenStream {
    let input = parse_macro_input!(attr as DeriveInput);

    let opts_name = &input.ident;

    let opts_cfg_gates = input
        .attrs
        .iter()
        .filter(|attr| {
            let Meta::List(list) = &attr.meta else { return false };
            list.path.is_ident("cfg")
        })
        .collect::<Vec<_>>();

    let Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) =
        input.data
    else {
        let msg = "expected a struct with named fields";
        return Error::new_spanned(input, msg).to_compile_error().into();
    };

    let Some(first_field) = fields.named.first() else {
        let msg = "expected at least one field";
        return Error::new_spanned(fields, msg).to_compile_error().into();
    };

    if !is_mask(first_field) {
        let msg = "expected the first field to have the `mask` attribute";
        return Error::new_spanned(first_field, msg).to_compile_error().into();
    }

    let mut opts_fields =
        fields.named.iter().skip(1).map(OptsField::from).collect::<Vec<_>>();

    let mut mask_ordering = (0..opts_fields.len()).collect::<Vec<_>>();

    mask_ordering.sort_by(|&left_idx, &right_idx| {
        let left_field = &opts_fields[left_idx];
        let right_field = &opts_fields[right_idx];

        let left_name = left_field.name.to_string();
        let right_name = right_field.name.to_string();

        match left_name.len().cmp(&right_name.len()) {
            Ordering::Equal => left_name.cmp(&right_name),
            ordering => ordering,
        }
    });

    for (mask_idx, opts_idx) in mask_ordering.into_iter().enumerate() {
        opts_fields[opts_idx].mask_idx = mask_idx;
    }

    let builder_name =
        Ident::new(&format!("{}Builder", opts_name), opts_name.span());

    let builder_method_doc_comment =
        format!("Creates a new [`{builder_name}`].");

    let mask_name = first_field.ident.as_ref().unwrap();

    let setters = opts_fields.iter().map(|field| {
        let field_doc_comment = &field.doc_comment;
        let field_mask_idx = &field.mask_idx;
        let field_name = &field.name;
        let field_ty = &field.ty;
        quote! {
            #[doc = #field_doc_comment]
            #[inline]
            pub fn #field_name(&mut self, #field_name: #field_ty) -> &mut Self {
                self.0.#field_name = #field_name;
                self.0.#mask_name |= (1 << (#field_mask_idx + 1)) + 1;
                self
            }
        }
    });

    quote! {
        #(#opts_cfg_gates)*
        impl #opts_name {
            #[doc = #builder_method_doc_comment]
            #[inline(always)]
            pub fn builder() -> #builder_name {
                #builder_name::default()
            }
        }

        #(#opts_cfg_gates)*
        pub struct #builder_name(#opts_name);

        #(#opts_cfg_gates)*
        impl ::core::clone::Clone for #builder_name {
            #[inline]
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        #(#opts_cfg_gates)*
        impl ::core::default::Default for #builder_name {
            #[inline]
            fn default() -> Self {
                Self(#opts_name::default())
            }
        }

        #(#opts_cfg_gates)*
        impl #builder_name {
            #(#setters)*

            #[inline]
            pub fn build(&mut self) -> #opts_name {
                ::core::mem::take(&mut self.0)
            }
        }
    }
    .into()
}

/// Returns `true` if the field has the `mask` attribute.
fn is_mask(field: &Field) -> bool {
    for attr in &field.attrs {
        let Meta::List(list) = &attr.meta else { continue };

        if !list.path.is_ident("builder") {
            continue;
        }

        let mut tokens = list.tokens.clone().into_iter();

        let Some(TokenTree::Ident(first_token)) = tokens.next() else {
            continue;
        };

        if first_token != "mask" {
            continue;
        }

        if tokens.next().is_some() {
            continue;
        }

        return true;
    }

    false
}

/// Returns the doc comment of the field, if any.
fn parse_doc_comment(field: &Field) -> Option<String> {
    for attr in &field.attrs {
        let Meta::NameValue(name_value) = &attr.meta else { continue };

        let Expr::Lit(ExprLit { lit: Lit::Str(doc_comment), .. }) =
            &name_value.value
        else {
            continue;
        };

        return Some(doc_comment.value());
    }

    None
}
