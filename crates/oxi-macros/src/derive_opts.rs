use proc_macro::TokenStream;
use syn::*;

#[derive(Debug)]
struct OptsField {
    name: Ident,
    doc_comment: Option<String>,
}

impl From<Field> for OptsField {
    fn from(field: Field) -> Self {
        Self {
            name: field.ident.unwrap(),
            doc_comment: parse_doc_comment(&field.attrs),
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

    let fields =
        fields.named.into_iter().map(OptsField::from).collect::<Vec<_>>();

    println!("{:#?}", fields);

    TokenStream::default()
}

fn parse_doc_comment(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
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
