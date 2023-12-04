use proc_macro::TokenStream;
use quote::quote;
use syn::*;

struct OptsField {
    name: Ident,
    ty: Type,
    doc_comment: Option<String>,
}

impl From<&Field> for OptsField {
    fn from(field: &Field) -> Self {
        Self {
            name: field.ident.as_ref().unwrap().clone(),
            ty: field.ty.clone(),
            doc_comment: parse_doc_comment(field),
        }
    }
}

/// TODO: docs
pub fn derive_opts_builder(attr: TokenStream) -> TokenStream {
    let input = parse_macro_input!(attr as DeriveInput);

    let struct_name = &input.ident;

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

    let builder_name =
        Ident::new(&format!("{}Builder", struct_name), struct_name.span());

    let builder_method_doc_comment =
        format!("Creates a new [`{builder_name}`].");

    let setters = opts_fields.iter().map(|field| {
        let field_doc_comment = &field.doc_comment;
        let field_name = &field.name;
        let field_ty = &field.ty;
        quote! {
            #[doc = #field_doc_comment]
            #[inline]
            pub fn #field_name(&mut self, #field_name: #field_ty) -> &mut Self {
                /// TODO: update the field and the mask
                self
            }
        }
    });

    quote! {
        impl #struct_name {
            #[doc = #builder_method_doc_comment]
            #[inline(always)]
            pub fn builder() -> #builder_name {
                #builder_name::default()
            }
        }

        #[derive(Clone, Default)]
        pub struct #builder_name(#struct_name);

        impl #builder_name {
            #(#setters)*
        }
    }
    .into()
}

/// Returns `true` if the field has the `mask` attribute.
fn is_mask(field: &Field) -> bool {
    for attr in &field.attrs {
        let Meta::Path(path) = &attr.meta else { continue };

        if path.is_ident("mask") {
            return true;
        }
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
