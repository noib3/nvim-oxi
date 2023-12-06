use core::cmp::Ordering;

use proc_macro2::{Span, TokenStream, TokenTree};
use quote::quote;
use syn::*;

const MASK_ATTR_IDENT: &str = "mask";

const INTO_ATTR_IDENT: &str = "Into";

const SETTER_ATTR_IDENT: &str = "setter";

/// TODO: docs
pub fn expand_derive_opts_builder(input: &DeriveInput) -> Result<TokenStream> {
    let opts_fields = OptsFields::try_from(input)?;
    let opts_builder = OptsBuilder::from(input);
    let cfg_gates = CfgGates::from(input);

    let opts = &input.ident;
    let opts_setters = opts_fields.setters();

    let builder = opts_builder.name();
    let builder_impl_clone = opts_builder.impl_clone();
    let builder_impl_default = opts_builder.impl_default();
    let builder_method_doc_comment = format!("Creates a new [`{builder}`].");

    let tokens = quote! {
        #cfg_gates
        impl #opts {
            #[doc = #builder_method_doc_comment]
            #[inline]
            pub fn builder() -> #builder {
                #builder::default()
            }
        }

        #cfg_gates
        pub struct #builder(#opts);

        #cfg_gates
        #builder_impl_clone

        #cfg_gates
        #builder_impl_default

        #cfg_gates
        impl #builder {
            #(#opts_setters)*

            #[inline]
            pub fn build(&mut self) -> #opts {
                ::core::mem::take(&mut self.0)
            }
        }
    };

    Ok(tokens)
}

/// TODO: docs
struct OptsBuilder<'a> {
    opts_name: &'a Ident,
    this_name: Ident,
}

impl<'a> From<&'a DeriveInput> for OptsBuilder<'a> {
    #[inline]
    fn from(input: &'a DeriveInput) -> Self {
        let opts_name = &input.ident;

        let this_name =
            Ident::new(&(opts_name.to_string() + "Builder"), opts_name.span());

        Self { opts_name, this_name }
    }
}

impl<'a> OptsBuilder<'a> {
    /// Returns the `impl Clone` block for the builder.
    #[inline]
    fn impl_clone(&self) -> TokenStream {
        let builder_name = self.name();
        quote! {
            impl ::core::clone::Clone for #builder_name {
                #[inline]
                fn clone(&self) -> Self {
                    Self(self.0.clone())
                }
            }
        }
    }

    /// Returns the `impl Default` block for the builder.
    #[inline]
    fn impl_default(&self) -> TokenStream {
        let builder_name = self.name();
        let opts_name = self.opts_name;
        quote! {
            impl ::core::default::Default for #builder_name {
                #[inline]
                fn default() -> Self {
                    Self(#opts_name::default())
                }
            }
        }
    }

    /// Returns the name of the builder.
    #[inline]
    fn name(&self) -> &Ident {
        &self.this_name
    }
}

/// TODO: docs
struct CfgGates<'a> {
    attrs: Vec<&'a Attribute>,
}

impl<'a> From<&'a DeriveInput> for CfgGates<'a> {
    #[inline]
    fn from(input: &'a DeriveInput) -> Self {
        let attrs = input
            .attrs
            .iter()
            .filter(|attr| {
                let Meta::List(list) = &attr.meta else { return false };
                list.path.is_ident("cfg")
            })
            .collect();

        Self { attrs }
    }
}

impl quote::ToTokens for CfgGates<'_> {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for attr in &self.attrs {
            attr.to_tokens(tokens);
        }
    }
}

/// TODO: docs
struct OptsFields<'a> {
    mask_name: &'a Ident,
    fields: Vec<OptsField<'a>>,
}

impl<'a> TryFrom<&'a DeriveInput> for OptsFields<'a> {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a DeriveInput) -> Result<Self> {
        let Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) =
            &input.data
        else {
            let msg = "expected a struct with named fields";
            return Err(Error::new_spanned(input, msg));
        };

        let Some(first_field) = fields.named.first() else {
            let msg = "expected at least one field";
            return Err(Error::new_spanned(fields, msg));
        };

        if !has_mask_attribute(first_field)? {
            let msg = "expected the first field to have the `builder(mask)` \
                       attribute";
            return Err(Error::new_spanned(first_field, msg));
        }

        let mask_name = first_field.ident.as_ref().unwrap();

        let mut opts_fields = fields
            .named
            .iter()
            .skip(1)
            .map(OptsField::try_from)
            .collect::<Result<Vec<_>>>()?;

        // We sort the based on their name so that the generated setter methods
        // will be in alphabetical order.
        opts_fields.sort_by_key(|field| field.name.to_string());

        let mut mask_ordering = (0..opts_fields.len()).collect::<Vec<_>>();

        mask_ordering.sort_by(|&left_idx, &right_idx| {
            let left_field = &opts_fields[left_idx];
            let right_field = &opts_fields[right_idx];
            left_field.mask_cmp(right_field)
        });

        for (mask_idx, opts_idx) in mask_ordering.into_iter().enumerate() {
            opts_fields[opts_idx].set_mask_idx(mask_idx);
        }

        Ok(Self { mask_name, fields: opts_fields })
    }
}

impl<'a> OptsFields<'a> {
    #[inline]
    fn setters(&self) -> impl Iterator<Item = TokenStream> + '_ {
        self.fields.iter().map(|field| field.setter(self.mask_name))
    }
}

/// TODO: docs
struct OptsField<'a> {
    attr: Option<BuilderAttribute>,
    doc_comment: Option<String>,
    mask_idx: usize,
    name: &'a Ident,
    ty: &'a Type,
}

impl<'a> TryFrom<&'a Field> for OptsField<'a> {
    type Error = Error;

    fn try_from(field: &'a Field) -> Result<Self> {
        let mut builder_attr = None;

        for field_attr in &field.attrs {
            if let Some(attr) = BuilderAttribute::try_from_attr(field_attr)? {
                if builder_attr.is_some() {
                    let msg = "expected at most one `builder(...)` attribute";
                    return Err(Error::new_spanned(field_attr, msg));
                } else if matches!(attr, BuilderAttribute::Mask) {
                    let msg = "the `builder(mask)` attribute can only be \
                               applied the first field";
                    return Err(Error::new_spanned(field_attr, msg));
                } else {
                    builder_attr = Some(attr);
                }
            }
        }

        Ok(Self {
            attr: builder_attr,
            doc_comment: parse_doc_comment(field),
            mask_idx: 0,
            name: field.ident.as_ref().unwrap(),
            ty: &field.ty,
        })
    }
}

impl<'a> OptsField<'a> {
    /// Returns the ordering that Neovim uses when comparing fields in the
    /// mask.
    #[inline]
    fn mask_cmp(&self, other: &Self) -> Ordering {
        let this_name = self.name.to_string();
        let other_name = other.name.to_string();

        match this_name.len().cmp(&other_name.len()) {
            Ordering::Equal => this_name.cmp(&other_name),
            ordering => ordering,
        }
    }

    /// TODO: docs
    #[inline]
    fn setter(&self, mask_name: &Ident) -> TokenStream {
        let field_name = &self.name;

        let mut generics: Option<TokenStream> = None;

        let where_clause: Option<TokenStream> = None;

        let mut field_type = self.ty.clone();

        let mut field_setter = quote! {
            self.0.#field_name = #field_name;
        };

        match &self.attr {
            Some(BuilderAttribute::Into) => {
                let generic_char = field_name
                    .to_string()
                    .chars()
                    .next()
                    .unwrap()
                    .to_ascii_uppercase();

                let generic_name =
                    Ident::new(&generic_char.to_string(), Span::call_site());

                generics = Some(
                    quote! { #generic_name: ::core::convert::Into<#field_type> },
                );

                field_type = Type::Verbatim(quote! { #generic_name });

                field_setter = quote! {
                    self.0.#field_name = #field_name.into();
                };
            },

            Some(BuilderAttribute::CustomSetterFn {
                function_name: _,
                setter_argument_ty: _,
                setter_argument_generic: _,
            }) => todo!(),

            Some(BuilderAttribute::Mask) => unreachable!(),

            None => {},
        };

        let field_doc_comment = &self.doc_comment;

        let field_mask_idx = &self.mask_idx;

        quote! {
            #[doc = #field_doc_comment]
            #[inline]
            pub fn #field_name<#generics>(
                &mut self,
                #field_name: #field_type,
            ) -> &mut Self
                #where_clause
            {
                #field_setter
                self.0.#mask_name |= (1 << (#field_mask_idx + 1)) + 1;
                self
            }
        }
    }

    /// Sets the index of the field in the mask. A value of `0` means that the
    /// field is the first in the mask, `1` means that it's the second, and so
    /// on.
    #[inline]
    fn set_mask_idx(&mut self, mask_idx: usize) {
        self.mask_idx = mask_idx;
    }
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

/// Returns `true` if the field has the `builder(mask)` attribute.
fn has_mask_attribute(field: &Field) -> Result<bool> {
    for attr in &field.attrs {
        let Some(BuilderAttribute::Mask) =
            BuilderAttribute::try_from_attr(attr)?
        else {
            continue;
        };

        return Ok(true);
    }

    Ok(false)
}

/// This enum represents the possible valid annotations that can be contained
/// inside the `#[builder(...)]` attribute of a field of a struct deriving
/// `OptsBuilder`.
enum BuilderAttribute {
    /// The `builder(mask)` attribute.
    ///
    /// This attribute is required and must be the only attribute present on
    /// the first field of the struct.
    Mask,

    /// The `builder(Into)` attribute.
    ///
    /// This attribute is optional and indicates that the setter should be
    /// generic over types implementing `Into<FieldType>`.
    Into,

    /// The `builder(setter = "fun")` attribute.
    ///
    /// This attribute is optional and will cause the setter to call the
    /// function `fun` with a mutable reference to the field and the argument
    /// of the setter.
    ///
    /// When the attribute is present, two more attrributes can be specified:
    ///
    /// - `builder(setter = "fun", arg = "Ty")` specifies the type of the
    ///   argument of the setter;
    ///
    /// - `builder(setter = "fun", arg = "Ty", generics = "Generics")`
    ///  specifies the generics to use for the setter.
    #[allow(dead_code)]
    CustomSetterFn {
        function_name: Ident,
        setter_argument_ty: Option<Type>,
        setter_argument_generic: Option<Generics>,
    },
}

impl BuilderAttribute {
    #[inline]
    fn try_from_attr(attr: &Attribute) -> Result<Option<Self>> {
        let Meta::List(list) = &attr.meta else {
            return Ok(None);
        };

        if !list.path.is_ident("builder") {
            return Ok(None);
        }

        let mut tokens = list.tokens.clone().into_iter();

        let Some(first_token) = tokens.next() else {
            let msg = "expected at least one token in the `builder` attribute";
            return Err(Error::new_spanned(list, msg));
        };

        let TokenTree::Ident(first_token) = first_token else {
            return Err(Error::new_spanned(
                first_token,
                "expected an identifier",
            ));
        };

        let this = if first_token == MASK_ATTR_IDENT {
            Self::Mask
        } else if first_token == INTO_ATTR_IDENT {
            Self::Into
        } else if first_token == SETTER_ATTR_IDENT {
            todo!();
        } else {
            let msg = "expected one of `mask`, `Into`, or `setter`";
            return Err(Error::new_spanned(first_token, msg));
        };

        if tokens.next().is_some() {
            let msg = "expected only one token in the `builder` attribute";
            return Err(Error::new_spanned(list, msg));
        }

        Ok(Some(this))
    }
}
