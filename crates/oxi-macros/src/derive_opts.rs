use proc_macro::TokenStream;
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

    let Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) =
        input.data
    else {
        let msg = "expected a struct with named fields";
        return Error::new_spanned(input, msg).to_compile_error().into();
    };

    let mut opts_fields = Vec::new();

    let mut mask_idx = None;

    for field in &fields.named {
        if is_mask(field) {
            if mask_idx.is_some() {
                let msg = "expected only one field with the `mask` attribute";
                return Error::new_spanned(field, msg)
                    .to_compile_error()
                    .into();
            }
            mask_idx = Some(opts_fields.len());
        }

        opts_fields.push(OptsField::from(field));
    }

    let Some(mask_idx) = mask_idx else {
        let msg = "expected a field with the `mask` attribute";
        return Error::new_spanned(fields, msg).to_compile_error().into();
    };

    TokenStream::default()
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
