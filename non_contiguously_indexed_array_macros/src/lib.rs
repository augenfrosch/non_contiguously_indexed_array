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

#[derive(Clone)]
enum Integer {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
}

impl TryFrom<&Expr> for Integer {
    type Error = syn::Error;
    fn try_from(value: &Expr) -> Result<Self> {
        // https://github.com/rust-phf/rust-phf/blob/1e518a6e94a2444b8df7d89078cfc69859537971/phf_macros/src/lib.rs#L138-L150
        macro_rules! negate_casted_if_needed {
            ($val:expr) => {
                if $val < 0 { $val } else { -$val }
            };
        }

        match value {
            Expr::Lit(expr_lit) => match &expr_lit.lit {
                syn::Lit::Int(lit_int) => match lit_int.suffix() {
                    "i8" => Ok(Self::I8(lit_int.base10_parse::<u8>()? as i8)),
                    "i16" => Ok(Self::I16(lit_int.base10_parse::<u16>()? as i16)),
                    "i32" => Ok(Self::I32(lit_int.base10_parse::<u32>()? as i32)),
                    "i64" => Ok(Self::I64(lit_int.base10_parse::<u64>()? as i64)),
                    "i128" => Ok(Self::I128(lit_int.base10_parse::<u128>()? as i128)),
                    "isize" => Ok(Self::Isize(lit_int.base10_parse::<usize>()? as isize)),
                    "u8" => Ok(Self::U8(lit_int.base10_parse::<u8>()?)),
                    "u16" => Ok(Self::U16(lit_int.base10_parse::<u16>()?)),
                    "u32" => Ok(Self::U32(lit_int.base10_parse::<u32>()?)),
                    "u64" => Ok(Self::U64(lit_int.base10_parse::<u64>()?)),
                    "u128" => Ok(Self::U128(lit_int.base10_parse::<u128>()?)),
                    "usize" => Ok(Self::Usize(lit_int.base10_parse::<usize>()?)),
                    _ => Err(syn::Error::new(
                        lit_int.span(),
                        "Unsupported literal! Literal must have a valid type suffix, e.g., `128u32`.",
                    )),
                },
                _ => Err(syn::Error::new(
                    expr_lit.span(),
                    "Unsupported literal! Literal must be for an integer.",
                )),
            },
            Expr::Unary(expr_unary) => match &expr_unary.op {
                syn::UnOp::Neg(minus) => match &*expr_unary.expr {
                    Expr::Lit(expr_lit) => match &expr_lit.lit {
                        syn::Lit::Int(lit_int) => match lit_int.suffix() {
                            "i8" => Ok(Self::I8(negate_casted_if_needed!(
                                lit_int.base10_parse::<u8>()? as i8
                            ))),
                            "i16" => Ok(Self::I16(negate_casted_if_needed!(
                                lit_int.base10_parse::<u16>()? as i16
                            ))),
                            "i32" => Ok(Self::I32(negate_casted_if_needed!(
                                lit_int.base10_parse::<u32>()? as i32
                            ))),
                            "i64" => Ok(Self::I64(negate_casted_if_needed!(
                                lit_int.base10_parse::<u64>()? as i64
                            ))),
                            "i128" => Ok(Self::I128(negate_casted_if_needed!(
                                lit_int.base10_parse::<u128>()? as i128
                            ))),
                            "isize" => Ok(Self::Isize(negate_casted_if_needed!(
                                lit_int.base10_parse::<usize>()? as isize
                            ))),
                            _ => Err(syn::Error::new(
                                lit_int.span(),
                                "Unsupported literal! Literal must be for a signed integer and must have a valid type suffix.",
                            )),
                        },
                        _ => Err(syn::Error::new(
                            expr_lit.span(),
                            "Unsupported literal! Literal must be for an integer.",
                        )),
                    },
                    _ => Err(syn::Error::new(
                        minus.span(),
                        "Unsupported unary expression! Only negation of integer literals is allowed.",
                    )),
                },
                _ => Err(syn::Error::new(
                    expr_unary.span(),
                    "Unsupported unary operator! Only negation, `-`, is allowed.",
                )),
            },
            _ => Err(syn::Error::new(
                value.span(),
                "Unsupported expression! Expression must be an integer literal or a negated signed integer literal.",
            )),
        }
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
    let mut values_exprs = Vec::with_capacity(input.len());

    macro_rules! check_pair_invariants {
        (($entry:ident, $next:ident),  $( $i:path ),*) => {
            match (&$entry.index.value, &$next.index.value) {
                $(($i(cur_idx), $i(nxt_idx)) => match nxt_idx.cmp(cur_idx) {
                    std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                        return TokenStream::from(syn::Error::new(
                            $next.index.expr.span(),
                            "The entries must be declared sorted from lowest to highest by their index without duplicates!"
                        ).into_compile_error());
                    },
                    std::cmp::Ordering::Greater => { },
                },)*
                _ => {
                    return TokenStream::from(syn::Error::new(
                        $next.index.expr.span(),
                        "Integer types must be consistent!",
                    ).into_compile_error());
                }
            }
        };
    }

    macro_rules! new_segment {
        (($previous:ident, $entry:ident),  $( $i:path ),*) => {
            match (&$previous.index.value, &$entry.index.value) {
                $(($i(prv_idx), $i(cur_idx)) =>
                    prv_idx.distance(*cur_idx) != Some(1),)*
                _ => unreachable!(), // must be used after `check_pair_invariants` to guarantee it is unreachable
            }
        };
    }

    for mem_idx in 0..input.len() {
        let new_segment = if mem_idx == 0 {
            true
        } else {
            let previous_entry = &input[mem_idx - 1];
            let current_entry = &input[mem_idx];
            check_pair_invariants!(
                (previous_entry, current_entry),
                Integer::I8,
                Integer::I16,
                Integer::I32,
                Integer::I64,
                Integer::I128,
                Integer::Isize,
                Integer::U8,
                Integer::U16,
                Integer::U32,
                Integer::U64,
                Integer::U128,
                Integer::Usize
            );
            new_segment!(
                (previous_entry, current_entry),
                Integer::I8,
                Integer::I16,
                Integer::I32,
                Integer::I64,
                Integer::I128,
                Integer::Isize,
                Integer::U8,
                Integer::U16,
                Integer::U32,
                Integer::U64,
                Integer::U128,
                Integer::Usize
            )
        };
        if new_segment {
            let index_expr = &input[mem_idx].index.expr;
            segments_idx_begin_exprs.push(index_expr);
            segments_mem_idx_begin.push(mem_idx);
        }
        values_exprs.push(&input[mem_idx].value);
    }

    // Further invariant checks can currently be skipped since they are always true for the integer types' `NciIndex` implementation
    // and explicit entries declaration via `index => value` ensures the last segment is valid.

    let nci_array = quote!(::non_contiguously_indexed_array::NciArray {
        segments_idx_begin: &[#(#segments_idx_begin_exprs),*],
        segments_mem_idx_begin: &[#(#segments_mem_idx_begin),*],
        values: &[#(#values_exprs),*],
    });

    TokenStream::from(nci_array)
}
