// Implementation is based on `phf_macros`

use non_contiguously_indexed_array_shared::NciIndex;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, Result, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
};

mod integer;
use integer::Integer;

impl TryFrom<&Expr> for Integer {
    type Error = syn::Error;
    fn try_from(value: &Expr) -> Result<Self> {
        match value {
            Expr::Lit(expr_lit) => match &expr_lit.lit {
                syn::Lit::Byte(lit_byte) => return Ok(Self::NonNegative(lit_byte.value().into())),
                syn::Lit::Int(lit_int) => {
                    return Ok(Self::NonNegative(lit_int.base10_parse::<u128>()?));
                }
                _ => {}
            },
            Expr::Unary(expr_unary) => {
                if let syn::UnOp::Neg(_minus) = &expr_unary.op
                    && let Expr::Lit(expr_lit) = &*expr_unary.expr
                    && let syn::Lit::Int(lit_int) = &expr_lit.lit
                {
                    return Ok(Self::Negative(lit_int.base10_parse::<u128>()?));
                }
            }
            _ => {}
        }

        Err(syn::Error::new(
            value.span(),
            "Unsupported expression! Expression must be a byte literal, an integer literal, or a negated signed integer literal.",
        ))
    }
}

#[derive(Clone)]
struct Index {
    expr: Expr,
    value: Integer,
}

impl Parse for Index {
    fn parse(input: ParseStream) -> Result<Self> {
        let expr = input.parse::<Expr>()?;
        let value = Integer::try_from(&expr)?;
        Ok(Self { value, expr })
    }
}

#[derive(Clone)]
struct Entry {
    index: Index,
    value: Expr,
    // attrs: Vec<syn::Attribute>,
}

impl Parse for Entry {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        if !attrs.is_empty() {
            return Err(syn::Error::new(
                attrs[0].span(),
                "The macro currently doesn't support attributes!",
            ));
        }

        let index = input.parse()?;
        input.parse::<Token![=>]>()?;
        let value = input.parse()?;
        Ok(Self {
            index,
            value,
            // attrs,
        })
    }
}

struct Entries(Punctuated<Entry, Token![,]>);

impl Parse for Entries {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed = Punctuated::<Entry, Token![,]>::parse_terminated(input)?;

        Ok(Self(parsed))
    }
}

#[proc_macro]
pub fn nci_array(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Entries).0;

    let mut segments_idx_begin_exprs = Vec::new();
    let mut segments_mem_idx_begin = Vec::new();
    let mut index_exprs = Vec::with_capacity(input.len());
    let mut values_exprs = Vec::with_capacity(input.len());

    for mem_idx in 0..input.len() {
        let new_segment = if mem_idx == 0 {
            true
        } else {
            let previous_entry = &input[mem_idx - 1];
            let current_entry = &input[mem_idx];
            match current_entry.index.value.cmp(&previous_entry.index.value) {
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                    return TokenStream::from(syn::Error::new(
                        current_entry.index.expr.span(),
                        "The entries must be declared sorted from lowest to highest by their index without duplicates!"
                    ).into_compile_error());
                }
                std::cmp::Ordering::Greater => {}
            }
            previous_entry.index.value.next() != Some(current_entry.index.value)
        };
        if new_segment {
            let index_expr = &input[mem_idx].index.expr;
            segments_idx_begin_exprs.push(index_expr);
            segments_mem_idx_begin.push(mem_idx);
        }
        index_exprs.push(&input[mem_idx].index.expr);
        values_exprs.push(&input[mem_idx].value);
    }

    // Further invariant checks can currently be skipped since they are always true for the integer types' `NciIndex` implementation
    // and explicit entries declaration via `index => value` ensures the last segment is valid.

    let nci_array = if let Some((segments_first_idx_begin_expr, segments_idx_begin_exprs)) =
        segments_idx_begin_exprs.split_first()
    {
        quote!({
            struct S<T>(T);
            impl<T> S<T> {
                const fn check_types(self, _: &[T]) -> Self {
                    self
                }
            }
            #[deny(overflowing_literals)]
            ::non_contiguously_indexed_array::NciArray {
                segments_idx_begin: &[S(#segments_first_idx_begin_expr).check_types(&[#(#index_exprs),*]).0, #(#segments_idx_begin_exprs),*],
                segments_mem_idx_begin: &[#(#segments_mem_idx_begin),*],
                values: &[#(#values_exprs),*],
            }
        })
    } else {
        quote!(::non_contiguously_indexed_array::NciArray::new())
    };

    TokenStream::from(nci_array)
}
