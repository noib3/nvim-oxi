use proc_macro2::*;
use quote::quote;
use syn::*;

/// TODO: docs
const INLINE_PLACEHOLDER: &str = "{0}";

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
#[derive(Default)]
struct OptsFields<'a> {
    mask_name: Option<&'a Ident>,
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
            return Ok(Self::default());
        };

        let mask_name = has_mask_attribute(first_field)?
            .then(|| first_field.ident.as_ref().unwrap());

        let mut opts_fields = fields
            .named
            .iter()
            .skip(if mask_name.is_some() { 1 } else { 0 })
            .map(OptsField::try_from)
            .collect::<Result<Vec<_>>>()?;

        let mask_ordering = hashy::hashy_hash(
            opts_fields.iter().map(|field| &*field.name_as_str),
        );

        for (mask_idx, opts_idx) in mask_ordering.into_iter().enumerate() {
            opts_fields[opts_idx].set_mask_idx(mask_idx);
        }

        // We sort the fields so that the generated setter methods will be in
        // alphabetical order.
        opts_fields
            .sort_by(|left, right| left.name_as_str.cmp(&right.name_as_str));

        Ok(Self { mask_name, fields: opts_fields })
    }
}

impl<'a> OptsFields<'a> {
    #[inline]
    fn setters(&self) -> impl Iterator<Item = TokenStream> + '_ {
        self.fields.iter().filter_map(|field| field.setter(self.mask_name))
    }
}

/// TODO: docs
struct OptsField<'a> {
    attrs: Vec<BuilderAttribute>,
    doc_comment: Option<String>,
    mask_idx: usize,
    name: &'a Ident,
    name_as_str: String,
    ty: &'a Type,
}

impl<'a> TryFrom<&'a Field> for OptsField<'a> {
    type Error = Error;

    fn try_from(field: &'a Field) -> Result<Self> {
        let attrs = field
            .attrs
            .iter()
            .filter_map(|attr| {
                let Meta::List(list) = &attr.meta else {
                    return None;
                };

                if !list.path.is_ident("builder") {
                    return None;
                }

                Some(list.tokens.clone().into_iter())
            })
            .flat_map(|mut tokens| {
                let mut attrs = Vec::new();
                while let Some(attr) =
                    BuilderAttribute::from_token_stream(&mut tokens)
                        .transpose()
                {
                    attrs.push(attr);
                }
                attrs
            })
            .collect::<Result<Vec<_>>>()?;

        is_valid_combination(&attrs)?;

        let name = field.ident.as_ref().unwrap();

        Ok(Self {
            attrs,
            doc_comment: parse_doc_comment(field),
            mask_idx: 0,
            name,
            name_as_str: name.to_string(),
            ty: &field.ty,
        })
    }
}

impl<'a> OptsField<'a> {
    /// TODO: docs
    #[inline]
    fn setter(&self, mask_name: Option<&Ident>) -> Option<TokenStream> {
        let field_name = self.name;

        let method_name = self
            .attrs
            .iter()
            .find_map(|attr| {
                if let BuilderAttribute::Method(method_name) = attr {
                    Some(method_name)
                } else {
                    None
                }
            })
            .unwrap_or(field_name);

        let argument_name = method_name;

        let mut generics: Option<TokenStream> = None;

        let mut field_type = self.ty.clone();

        let mut field_setter = quote! {
            self.0.#field_name = #argument_name;
        };

        for attr in &self.attrs {
            match &attr {
                BuilderAttribute::ArgType(arg_type) => {
                    field_type = arg_type.clone();
                },

                BuilderAttribute::Generics(gen) => {
                    let gen = gen.clone();
                    generics = Some(quote! { #gen });
                },

                BuilderAttribute::Inline(inline) => {
                    let placeholder_start =
                        inline.find(INLINE_PLACEHOLDER).unwrap();

                    let placeholder_end =
                        placeholder_start + INLINE_PLACEHOLDER.len();

                    let inline_expr_str = format!(
                        "{before}{argument_name}{after}",
                        before = &inline[..placeholder_start],
                        after = &inline[placeholder_end..],
                    );

                    let inline_expr =
                        parse_str::<Expr>(&inline_expr_str).unwrap();

                    field_setter = quote! {
                        self.0.#field_name = #inline_expr;
                    };
                },

                BuilderAttribute::Into => {
                    let generic_name = argument_name
                        .to_string()
                        .chars()
                        .next()
                        .unwrap()
                        .to_ascii_uppercase()
                        .to_string();

                    let generic = Ident::new(&generic_name, Span::call_site());

                    generics = Some(quote! {
                        #generic: ::core::convert::Into<#field_type>
                    });

                    field_type = Type::Verbatim(quote! { #generic });

                    field_setter = quote! {
                        self.0.#field_name = #argument_name.into();
                    };
                },

                BuilderAttribute::Method(_) => {},

                BuilderAttribute::Setter(setter_fn) => {
                    field_setter = quote! {
                        #setter_fn(&mut self.0.#field_name, #argument_name);
                    };
                },

                BuilderAttribute::Mask => unreachable!(),

                BuilderAttribute::Skip => return None,
            }
        }

        let field_doc_comment = self.doc_comment.as_ref().map(|docs| {
            quote! { #[doc = #docs] }
        });

        let mask_setter = mask_name.map(|mask_name| {
            let field_mask_idx = &self.mask_idx;
            quote! {
                self.0.#mask_name |= 1 << (#field_mask_idx + 1);
            }
        });

        let tokens = quote! {
            #field_doc_comment
            #[inline]
            pub fn #method_name #generics(
                &mut self,
                #argument_name: #field_type,
            ) -> &mut Self {
                #field_setter
                #mask_setter
                self
            }
        };

        Some(tokens)
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
        let Meta::List(list) = &attr.meta else {
            continue;
        };

        if !list.path.is_ident("builder") {
            continue;
        }

        let mut tokens = list.tokens.clone().into_iter();

        let Some(attr) = BuilderAttribute::from_token_stream(&mut tokens)?
        else {
            return Err(Error::new_spanned(
                list,
                "expected a `builder(mask)` attribute",
            ));
        };

        if matches!(attr, BuilderAttribute::Mask) {
            if tokens.next().is_some() {
                let msg = "expected no tokens after `builder(mask)`";
                return Err(Error::new_spanned(list, msg));
            }
            return Ok(true);
        }
    }

    Ok(false)
}

/// This enum represents the possible valid annotations that can be contained
/// inside the `#[builder(...)]` attribute of a field of a struct deriving
/// `OptsBuilder`.
enum BuilderAttribute {
    /// The `builder(argtype = "<type>")` attribute.
    ///
    /// TODO: docs
    ArgType(Type),

    /// The `builder(generics = "<generics>")` attribute.
    ///
    /// TODO: docs
    Generics(Generics),

    /// The `builder(inline = "<expr>")` attribute.
    ///
    /// TODO: docs
    Inline(String),

    /// The `builder(Into)` attribute.
    ///
    /// This attribute is optional and indicates that the setter should be
    /// generic over types implementing `Into<FieldType>`.
    Into,

    /// The `builder(mask)` attribute.
    ///
    /// This attribute is required and must be the only attribute present on
    /// the first field of the struct.
    Mask,

    /// The `builder(method = "<name>")` attribute.
    ///
    /// This attribute is optional and it can be used to override the name of
    /// the setter method (the default name is the name of the field).
    Method(Ident),

    /// The `builder(setter = "<fun>")` attribute.
    ///
    /// This attribute is optional and will cause the setter to call the
    /// function `fun` with a mutable reference to the field and the argument
    /// of the setter.
    Setter(Ident),

    /// The `builder(skip)` attribute.
    ///
    /// This attribute is optional and it can be used to avoid creating the
    /// setter method for a given field.
    Skip,
}

impl BuilderAttribute {
    #[inline]
    fn from_token_stream(
        tokens: &mut token_stream::IntoIter,
    ) -> Result<Option<Self>> {
        let Some(token) = tokens.next() else { return Ok(None) };

        let TokenTree::Ident(ident) = token else {
            let msg = "expected an identifier";
            return Err(Error::new_spanned(token, msg));
        };

        let mut is_argtype = false;
        let mut is_generics = false;
        let mut is_inline = false;
        let mut is_method = false;
        let mut is_setter = false;

        if ident == "Into" {
            // Consume the `,` (if any).
            let _ = tokens.next();
            return Ok(Some(Self::Into));
        } else if ident == "mask" {
            // Consume the `,` (if any).
            let _ = tokens.next();
            return Ok(Some(Self::Mask));
        } else if ident == "skip" {
            // Consume the `,` (if any).
            let _ = tokens.next();
            return Ok(Some(Self::Skip));
        } else if ident == "argtype" {
            is_argtype = true;
        } else if ident == "generics" {
            is_generics = true;
        } else if ident == "inline" {
            is_inline = true;
        } else if ident == "method" {
            is_method = true;
        } else if ident == "setter" {
            is_setter = true;
        } else {
            let msg = format!("unknown attribute `{}`", ident);
            return Err(Error::new_spanned(ident, msg));
        }

        let Some(TokenTree::Punct(punct)) = tokens.next() else {
            let msg = format!("expected a `=` after `{ident:?}`");
            return Err(Error::new_spanned(ident, msg));
        };

        if punct.as_char() != '=' {
            let msg = format!("expected a `=` after `{ident}`");
            return Err(Error::new_spanned(ident, msg));
        }

        let Some(TokenTree::Literal(lit)) = tokens.next() else {
            let msg = format!("expected a string literal after `{ident} = `",);
            return Err(Error::new_spanned(ident, msg));
        };

        let lit = lit.to_string();

        let lit = if lit.starts_with('r') {
            // Remove the leading `r#"` and the trailing `"#`.
            lit[3..lit.len() - 2].to_owned()
        } else if lit.starts_with('"') {
            // Remove the enclosing double quotes.
            lit[1..lit.len() - 1].to_owned()
        } else {
            unimplemented!();
        };

        let this = if is_argtype {
            parse_str(&lit).map(Self::ArgType)
        } else if is_generics {
            let lit = format!("<{lit}>");
            parse_str(&lit).map(Self::Generics)
        } else if is_inline {
            Ok(Self::Inline(lit))
        } else if is_method {
            parse_str(&lit).map(Self::Method)
        } else if is_setter {
            parse_str(&lit).map(Self::Setter)
        } else {
            unreachable!()
        }
        .map(Some);

        // Consume the `,` (if any).
        let _ = tokens.next();

        this
    }
}

/// Returns `Ok(())` if the given combination of attributes is valid, otherwise
/// returns an error.
#[inline]
fn is_valid_combination(attrs: &[BuilderAttribute]) -> Result<()> {
    // Invariants to check:
    //
    // 1. an attribute can only be present once;
    // 2. `Mask` and `Into` are only valid if they're the only attribute;
    // 3. `Generics` and `Into` are mutually exclusive;
    // 4. `Inline` and `Setter` are mutually exclusive;
    // 5. if `Generics` is present, `ArgType` must also be present;

    let mut has_argtype = false;
    let mut has_generics = false;
    let mut has_inline = false;
    let mut has_into = false;
    let mut has_mask = false;
    let mut has_method = false;
    let mut has_setter = false;
    let mut has_skip = false;

    for attr in attrs {
        let is_duplicate;

        match attr {
            BuilderAttribute::ArgType(_) => {
                is_duplicate = has_argtype;
                has_argtype = true;
            },

            BuilderAttribute::Generics(_) => {
                is_duplicate = has_generics;
                has_generics = true;
            },

            BuilderAttribute::Inline(inline) => {
                if !inline.contains(INLINE_PLACEHOLDER) {
                    let _msg = format!(
                        "expected `{}` in the expression of the `inline` \
                         attribute",
                        INLINE_PLACEHOLDER,
                    );
                    todo!();
                }

                is_duplicate = has_inline;
                has_inline = true;
            },

            BuilderAttribute::Into => {
                is_duplicate = has_into;
                has_into = true;
            },

            BuilderAttribute::Mask => {
                is_duplicate = has_mask;
                has_mask = true;
            },

            BuilderAttribute::Method(_) => {
                is_duplicate = has_method;
                has_method = true;
            },

            BuilderAttribute::Setter(_) => {
                is_duplicate = has_setter;
                has_setter = true;
            },

            BuilderAttribute::Skip => {
                is_duplicate = has_skip;
                has_skip = true;
            },
        }

        if is_duplicate {
            todo!();
        }
    }

    let has_mask_and_other = has_mask
        && (has_argtype
            || has_generics
            || has_inline
            || has_into
            || has_setter);

    if has_mask_and_other {
        todo!();
    }

    let has_into_and_other = has_into
        && (has_argtype
            || has_generics
            || has_inline
            || has_mask
            || has_setter);

    if has_into_and_other {
        todo!();
    }

    if has_generics && has_into {
        todo!();
    }

    if has_inline && has_setter {
        todo!();
    }

    if has_generics && !has_argtype {
        todo!();
    }

    Ok(())
}

mod hashy {
    //! This module re-implements [the logic][hashy] used by Neovim to sort the
    //! fields of the [keysets] structs when generating mask indices at compile
    //! time.
    //!
    //! The names of the various functions and variables don't make much sense
    //! in this context, but I've tried to keep the code as 1:1 as possible to
    //! the original to make it easier to compare them, and update this if/when
    //! upstream changes.
    //!
    //! [hashy]: https://github.com/neovim/neovim/blob/01c15a30c0ab56e14342f9996bea3ad86a68a343/src/nvim/generators/hashy.lua
    //! [keysets]: https://github.com/neovim/neovim/blob/01c15a30c0ab56e14342f9996bea3ad86a68a343/src/nvim/api/keysets_defs.h

    #![allow(clippy::needless_range_loop)]

    use core::hash::Hash;
    use std::collections::HashMap;

    type Bucket = HashMap<char, Vec<usize>>;

    pub(super) fn hashy_hash<'a>(
        fields: impl IntoIterator<Item = &'a str>,
    ) -> Vec<usize> {
        let fields = fields.into_iter().collect::<Vec<_>>();
        let (len_pos_buckets, maxlen) = build_pos_hash(&fields);
        switcher(len_pos_buckets, maxlen)
    }

    fn build_pos_hash(strings: &[&str]) -> (Vec<Bucket>, usize) {
        let mut len_buckets: Vec<Vec<usize>> = Vec::new();
        let mut maxlen = 0;

        for (idx, s) in strings.iter().enumerate() {
            setdefault_vec(&mut len_buckets, s.len()).push(idx);
            if s.len() > maxlen {
                maxlen = s.len();
            }
        }

        let mut len_pos_buckets = Vec::new();

        for len in 1..=maxlen {
            let strs = &len_buckets[len];

            if strs.is_empty() {
                len_pos_buckets.push(Bucket::new());
                continue;
            }

            let mut minsize = strs.len() * 2;
            let mut best_bucket = HashMap::new();

            for pos in 1..=len {
                let mut try_bucket = Bucket::new();
                for &str_idx in strs {
                    let str = &strings[str_idx];
                    let poschar = str.chars().nth(pos - 1).unwrap();
                    setdefault_map(&mut try_bucket, poschar).push(str_idx);
                }
                let mut maxsize = 1;
                for pos_strs in try_bucket.values() {
                    maxsize = maxsize.max(pos_strs.len());
                }
                if maxsize < minsize {
                    minsize = maxsize;
                    best_bucket = try_bucket;
                }
            }

            len_pos_buckets.push(best_bucket);
        }

        (len_pos_buckets, maxlen)
    }

    fn switcher(tab: Vec<Bucket>, maxlen: usize) -> Vec<usize> {
        let mut neworder = Vec::new();

        for len in 1..=maxlen {
            let posbuck = &tab[len - 1];
            if posbuck.is_empty() {
                continue;
            }
            let mut keys = posbuck.keys().collect::<Vec<_>>();
            if keys.len() > 1 {
                keys.sort();
                for c in keys {
                    let buck = &posbuck[c];
                    neworder.extend(buck);
                }
            } else {
                neworder.push(posbuck[keys[0]][0])
            }
        }

        neworder
    }

    fn setdefault_map<K, T: Default>(map: &mut HashMap<K, T>, key: K) -> &mut T
    where
        K: Eq + Hash,
    {
        map.entry(key).or_default()
    }

    fn setdefault_vec<T: Default>(vec: &mut Vec<T>, key: usize) -> &mut T {
        if vec.len() < key + 1 {
            vec.resize_with(key + 1, T::default);
        }
        &mut vec[key]
    }
}

#[cfg(test)]
mod tests {
    //! To get the correct order compile Neovim from source and look at the
    //! generated file in `/build/src/nvim/auto/keysets_defs.generated.h`.

    use super::*;

    fn order<'a>(fields: &[&'a str]) -> Vec<&'a str> {
        let order_idxs = hashy::hashy_hash(fields.iter().copied());

        let mut order = vec![""; fields.len()];

        for (order_idx, field_idx) in order_idxs.into_iter().enumerate() {
            order[order_idx] = fields[field_idx];
        }

        order
    }

    #[test]
    fn fields_order_highlight() {
        let fields = [
            "bold",
            "standout",
            "strikethrough",
            "underline",
            "undercurl",
            "underdouble",
            "underdotted",
            "underdashed",
            "italic",
            "reverse",
            "altfont",
            "nocombine",
            "default",
            "cterm",
            "foreground",
            "fg",
            "background",
            "bg",
            "ctermfg",
            "ctermbg",
            "special",
            "sp",
            "link",
            "global_link",
            "fallback",
            "blend",
            "fg_indexed",
            "bg_indexed",
            "force",
            "url",
        ];

        assert_eq!(
            order(&fields),
            [
                "bg",
                "fg",
                "sp",
                "url",
                "bold",
                "link",
                "blend",
                "cterm",
                "force",
                "italic",
                "special",
                "ctermbg",
                "ctermfg",
                "default",
                "altfont",
                "reverse",
                "fallback",
                "standout",
                "nocombine",
                "undercurl",
                "underline",
                "background",
                "bg_indexed",
                "foreground",
                "fg_indexed",
                "global_link",
                "underdashed",
                "underdotted",
                "underdouble",
                "strikethrough",
            ]
        );
    }

    #[test]
    fn fields_order_win_config() {
        let fields = [
            "row",
            "col",
            "width",
            "height",
            "anchor",
            "relative",
            "split",
            "win",
            "bufpos",
            "external",
            "focusable",
            "vertical",
            "zindex",
            "border",
            "title",
            "title_pos",
            "footer",
            "footer_pos",
            "style",
            "noautocmd",
            "fixed",
            "hide",
        ];

        assert_eq!(
            order(&fields),
            [
                "col",
                "row",
                "win",
                "hide",
                "width",
                "split",
                "title",
                "fixed",
                "style",
                "anchor",
                "bufpos",
                "height",
                "zindex",
                "footer",
                "border",
                "external",
                "relative",
                "vertical",
                "focusable",
                "noautocmd",
                "title_pos",
                "footer_pos"
            ]
        );
    }
}
